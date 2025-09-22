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
            Tags{ "LightMode"="UniversalForward" }
            HLSLPROGRAM
            #pragma target 3.0
            #pragma vertex vert
            #pragma fragment frag
            #pragma multi_compile_instancing
            #pragma multi_compile_fog
            #include "UnityCG.cginc"

            sampler2D _MainTex;
            float4 _MainTex_ST;
            sampler2D _PaintTex;

            #ifndef CBUFFER_START
            #define CBUFFER_START(name) cbuffer name {
            #define CBUFFER_END }
            #endif

            CBUFFER_START(UnityPerMaterial)
                float4 _BaseColor;
                float  _PaintIntensity;
                float  _Smoothness;
                float  _Metallic;
            CBUFFER_END;

            CBUFFER_START(UnityPerMainLight)
                float4 _MainLightPosition;
                float4 _MainLightColor;
            CBUFFER_END;

            struct Attributes
            {
                float4 positionOS : POSITION;
                float3 normalOS : NORMAL;
                float2 uv : TEXCOORD0;
                UNITY_VERTEX_INPUT_INSTANCE_ID
            };

            struct Varyings
            {
                float4 positionCS : SV_POSITION;
                float3 positionWS : TEXCOORD0;
                float3 normalWS : TEXCOORD1;
                float2 uv : TEXCOORD2;
                float3 viewDirWS : TEXCOORD3;
                UNITY_FOG_COORDS(4)
                UNITY_VERTEX_INPUT_INSTANCE_ID
                UNITY_VERTEX_OUTPUT_STEREO
            };

            Varyings vert(Attributes IN)
            {
                Varyings o;
                UNITY_SETUP_INSTANCE_ID(IN);
                UNITY_INITIALIZE_OUTPUT(Varyings, o);
                UNITY_TRANSFER_INSTANCE_ID(IN, o);
                UNITY_INITIALIZE_VERTEX_OUTPUT_STEREO(o);

                float4 worldPos = mul(unity_ObjectToWorld, IN.positionOS);
                o.positionCS = mul(UNITY_MATRIX_VP, worldPos);
                o.positionWS = worldPos.xyz;
                o.normalWS = UnityObjectToWorldNormal(IN.normalOS);
                o.uv = TRANSFORM_TEX(IN.uv, _MainTex);
                o.viewDirWS = _WorldSpaceCameraPos - o.positionWS;

                UNITY_TRANSFER_FOG(o, o.positionCS);
                return o;
            }

            float3 ComputeAmbient(float3 normalWS)
            {
                return ShadeSH9(float4(normalWS, 1.0)).rgb;
            }

            float3 GetMainLightDirection(float3 positionWS)
            {
                float3 lightVector = _MainLightPosition.xyz;
                if (_MainLightPosition.w == 0.0)
                {
                    return normalize(-lightVector);
                }
                else
                {
                    return normalize(lightVector - positionWS);
                }
            }

            float3 ComputeDiffuse(float3 diffuseColor, float3 normalWS, float3 lightDir, float3 lightColor)
            {
                float ndotl = saturate(dot(normalWS, lightDir));
                return diffuseColor * lightColor * ndotl;
            }

            float3 ComputeSpecular(float3 specColor, float3 normalWS, float3 viewDir, float3 lightDir, float3 lightColor, float smoothness)
            {
                float ndotl = saturate(dot(normalWS, lightDir));
                if (ndotl <= 0.0)
                    return 0;

                float3 halfDir = normalize(lightDir + viewDir);
                float ndoth = saturate(dot(normalWS, halfDir));
                float specPower = lerp(8.0, 512.0, saturate(smoothness));
                float spec = pow(ndoth, specPower);
                return specColor * lightColor * spec * ndotl;
            }

            half4 frag(Varyings IN) : SV_Target
            {
                UNITY_SETUP_INSTANCE_ID(IN);
                UNITY_SETUP_STEREO_EYE_INDEX_POST_VERTEX(IN);

                float3 normalWS = normalize(IN.normalWS);
                float3 viewDir = normalize(IN.viewDirWS);

                half4 baseAlbedo = tex2D(_MainTex, IN.uv) * _BaseColor;
                half4 paint = tex2D(_PaintTex, IN.uv);
                half mask = saturate(paint.a * _PaintIntensity);
                float3 albedo = lerp(baseAlbedo.rgb, paint.rgb, mask);

                float metallic = saturate(_Metallic);
                float smoothness = saturate(_Smoothness);
                float3 diffuseColor = albedo * (1.0 - metallic);
                float3 specColor = lerp(float3(0.04, 0.04, 0.04), albedo, metallic);

                float3 ambient = ComputeAmbient(normalWS) * diffuseColor;
#ifdef UNITY_LIGHTMODEL_AMBIENT
                ambient += UNITY_LIGHTMODEL_AMBIENT.rgb * diffuseColor;
#endif

                float3 lightDir = GetMainLightDirection(IN.positionWS);
                float3 lightColor = _MainLightColor.rgb;
                float3 diffuse = ComputeDiffuse(diffuseColor, normalWS, lightDir, lightColor);
                float3 specular = ComputeSpecular(specColor, normalWS, viewDir, lightDir, lightColor, smoothness);

                float3 color = ambient + diffuse + specular;
                half4 outColor = half4(saturate(color), 1.0);
                UNITY_APPLY_FOG(IN.fogCoord, outColor);
                return outColor;
            }
            ENDHLSL
        }
    }
}
