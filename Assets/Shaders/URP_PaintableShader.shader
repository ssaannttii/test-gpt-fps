Shader "Universal Render Pipeline/URP_Paintable"
{
    Properties
    {
        _BaseColor ("Base Color", Color) = (0.7,0.7,0.7,1)
        _MainTex ("Base Albedo (optional)", 2D) = "white" {}
        _PaintTex ("Paint Splatmap", 2D) = "black" {}
        _PaintIntensity ("Paint Intensity", Range(0,2)) = 1.0
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
            #pragma multi_compile_fog
            #include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/Core.hlsl"

            TEXTURE2D(_MainTex); SAMPLER(sampler_MainTex);
            TEXTURE2D(_PaintTex); SAMPLER(sampler_PaintTex);
            float4 _BaseColor;
            float  _PaintIntensity;

            struct Attributes { float4 positionOS:POSITION; float3 normalOS:NORMAL; float2 uv:TEXCOORD0; };
            struct Varyings  { float4 positionHCS:SV_POSITION; float3 normalWS:TEXCOORD1; float2 uv:TEXCOORD0; float3 posWS:TEXCOORD2; };

            Varyings vert(Attributes IN)
            {
                Varyings o;
                o.posWS = TransformObjectToWorld(IN.positionOS.xyz);
                o.positionHCS = TransformWorldToHClip(o.posWS);
                o.normalWS = TransformObjectToWorldNormal(IN.normalOS);
                o.uv = IN.uv;
                return o;
            }

            half4 frag(Varyings IN):SV_Target
            {
                half4 baseAlbedo = SAMPLE_TEXTURE2D(_MainTex, sampler_MainTex, IN.uv) * _BaseColor;
                half4 paint = SAMPLE_TEXTURE2D(_PaintTex, sampler_PaintTex, IN.uv);
                // paint.rgb = color acumulado; paint.a = intensidad acumulada
                half mask = saturate(paint.a * _PaintIntensity);
                half3 col = lerp(baseAlbedo.rgb, paint.rgb, mask);
                return half4(col, 1);
            }
            ENDHLSL
        }
    }
}
