Shader "Universal Render Pipeline/URP_Paintable"
{
    Properties
    {
        _BaseColor ("Base Color", Color) = (0.7,0.7,0.7,1)
        _MainTex ("Base Albedo (optional)", 2D) = "white" {}
        _PaintTex ("Paint Splatmap", 2D) = "black" {}
        _PaintIntensity ("Paint Intensity", Range(0,2)) = 1.0
        _Smoothness ("Smoothness", Range(0,1)) = 0.35
        _Metallic ("Metallic", Range(0,1)) = 0.0
    }
    SubShader
    {
        Tags { "Queue"="Geometry" "RenderType"="Opaque" "RenderPipeline"="UniversalPipeline"}
        LOD 200

        Pass
        {
            Name "ForwardLit"
            Tags{"LightMode"="UniversalForward"}
            HLSLPROGRAM
            #pragma vertex   vert
            #pragma fragment frag
            #pragma multi_compile _ _MAIN_LIGHT_SHADOWS
            #pragma multi_compile _ _ADDITIONAL_LIGHTS_VERTEX _ADDITIONAL_LIGHTS
            #pragma multi_compile _ _ADDITIONAL_LIGHT_SHADOWS
            #pragma multi_compile _ _SHADOWS_SOFT
            #pragma multi_compile _ _MIXED_LIGHTING_SUBTRACTIVE
            #pragma multi_compile_fragment _ _SCREEN_SPACE_OCCLUSION
            #pragma multi_compile _ LIGHTMAP_ON
            #pragma multi_compile _ DIRLIGHTMAP_COMBINED
            #pragma multi_compile _ DEBUG_DISPLAY
            #pragma multi_compile_instancing
            #pragma multi_compile_fog
            #include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/Core.hlsl"
            #include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/Lighting.hlsl"
            TEXTURE2D(_MainTex); SAMPLER(sampler_MainTex);
            TEXTURE2D(_PaintTex); SAMPLER(sampler_PaintTex);

            CBUFFER_START(UnityPerMaterial)
                float4 _BaseColor;
                float4 _MainTex_ST;
                float  _PaintIntensity;
                float  _Smoothness;
                float  _Metallic;
            CBUFFER_END

            float3x3 BuildTangentToWorld(float3 normalWS)
            {
                float3 tangentWS = normalize(cross(float3(0, 1, 0), normalWS));
                if (all(abs(tangentWS) < 1e-5))
                {
                    tangentWS = normalize(cross(float3(1, 0, 0), normalWS));
                }
                float3 bitangentWS = cross(normalWS, tangentWS);
                return float3x3(tangentWS, bitangentWS, normalWS);
            }

            struct Attributes
            {
                float4 positionOS : POSITION;
                float3 normalOS : NORMAL;
                float4 tangentOS : TANGENT;
                float2 uv : TEXCOORD0;
            #ifdef LIGHTMAP_ON
                float2 lightmapUV : TEXCOORD1;
            #endif
            #ifdef DYNAMICLIGHTMAP_ON
                float2 dynamicLightmapUV : TEXCOORD2;
            #endif
                UNITY_VERTEX_INPUT_INSTANCE_ID
            };

            struct Varyings
            {
                float4 positionCS : SV_POSITION;
                float3 positionWS : TEXCOORD0;
                half3 normalWS : TEXCOORD1;
                float2 uv : TEXCOORD2;
                float3 viewDirWS : TEXCOORD3;
                DECLARE_LIGHTMAP_OR_SH(lightmapUV, vertexSH, 4);
            #ifdef DYNAMICLIGHTMAP_ON
                float2 dynamicLightmapUV : TEXCOORD5;
            #endif
            #ifdef REQUIRES_SHADOW_COORD
                float4 shadowCoord : TEXCOORD6;
            #endif
                UNITY_VERTEX_INPUT_INSTANCE_ID
                UNITY_VERTEX_OUTPUT_STEREO
            };

            Varyings vert(Attributes IN)
            {
                Varyings o;
                UNITY_SETUP_INSTANCE_ID(IN);
                UNITY_INITIALIZE_VERTEX_OUTPUT_STEREO(o);
                UNITY_TRANSFER_INSTANCE_ID(IN, o);

                VertexPositionInputs positionInputs = GetVertexPositionInputs(IN.positionOS.xyz);
                VertexNormalInputs normalInputs = GetVertexNormalInputs(IN.normalOS, IN.tangentOS);

                o.positionCS = positionInputs.positionCS;
                o.positionWS = positionInputs.positionWS;
                o.normalWS = NormalizeNormalPerVertex(normalInputs.normalWS);
                o.uv = TRANSFORM_TEX(IN.uv, _MainTex);
                o.viewDirWS = GetWorldSpaceViewDir(o.positionWS);

#ifdef LIGHTMAP_ON
                OUTPUT_LIGHTMAP_UV(IN.lightmapUV, unity_LightmapST, o.lightmapUV);
#else
                OUTPUT_SH(o.normalWS, o.vertexSH);
#endif

#ifdef DYNAMICLIGHTMAP_ON
                o.dynamicLightmapUV = IN.dynamicLightmapUV * unity_DynamicLightmapST.xy + unity_DynamicLightmapST.zw;
#endif

#ifdef REQUIRES_SHADOW_COORD
                o.shadowCoord = GetShadowCoord(positionInputs);
#endif

                return o;
            }

            half4 frag(Varyings IN) : SV_Target
            {
                UNITY_SETUP_INSTANCE_ID(IN);
                UNITY_SETUP_STEREO_EYE_INDEX_POST_VERTEX(IN);

                half3 normalWS = NormalizeNormalPerPixel(IN.normalWS);

                InputData inputData = (InputData)0;
                inputData.positionWS = IN.positionWS;
                inputData.normalWS = normalWS;
                inputData.viewDirectionWS = SafeNormalize(IN.viewDirWS);
#ifdef REQUIRES_SHADOW_COORD
                inputData.shadowCoord = IN.shadowCoord;
#else
                inputData.shadowCoord = float4(0, 0, 0, 0);
#endif
                inputData.fogCoord = ComputeFogFactor(IN.positionCS.z);
                inputData.vertexLighting = half3(0, 0, 0);
                inputData.bakedGI = SAMPLE_GI(IN.lightmapUV, IN.vertexSH, normalWS);
                inputData.shadowMask = SAMPLE_SHADOWMASK(IN.lightmapUV);
                inputData.normalizedScreenSpaceUV = GetNormalizedScreenSpaceUV(IN.positionCS);
                float3x3 t2w = BuildTangentToWorld(normalWS);
                inputData.tangentToWorld = half3x3(
                    half3(t2w[0].x, t2w[0].y, t2w[0].z),
                    half3(t2w[1].x, t2w[1].y, t2w[1].z),
                    half3(t2w[2].x, t2w[2].y, t2w[2].z));
                inputData.occlusion = 1;

                half4 baseAlbedo = SAMPLE_TEXTURE2D(_MainTex, sampler_MainTex, IN.uv) * _BaseColor;
                half4 paint = SAMPLE_TEXTURE2D(_PaintTex, sampler_PaintTex, IN.uv);
                half mask = saturate(paint.a * _PaintIntensity);
                half3 albedo = lerp(baseAlbedo.rgb, paint.rgb, mask);

                SurfaceData surfaceData;
                surfaceData.albedo = albedo;
                surfaceData.specular = half3(0, 0, 0);
                surfaceData.metallic = _Metallic;
                surfaceData.smoothness = _Smoothness;
                surfaceData.normalTS = half3(0, 0, 1);
                surfaceData.occlusion = 1;
                surfaceData.emission = half3(0, 0, 0);
                surfaceData.alpha = 1;

                half4 color = UniversalFragmentPBR(inputData, surfaceData);
                color.rgb = MixFog(color.rgb, inputData.fogCoord);
                return color;
            }
            ENDHLSL
        }
    }
}
