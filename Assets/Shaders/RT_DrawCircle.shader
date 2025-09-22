Shader "Hidden/RT_DrawCircle"
{
    Properties{}
    SubShader
    {
        Tags{ "RenderType"="Opaque" "Queue"="Overlay" }
        Cull Off ZWrite Off ZTest Always
        Pass
        {
            HLSLPROGRAM
            #pragma vertex vert
            #pragma fragment frag
            #include "UnityCG.cginc"

            struct appdata { float4 vertex:POSITION; float2 uv:TEXCOORD0; };
            struct v2f { float4 pos:SV_POSITION; float2 uv:TEXCOORD0; };

            sampler2D _MainTex; // RT actual
            float4 _BrushUVR;   // uvx, uvy, radiusUV, _
            float4 _BrushColor; // rgb = color
            float  _Hardness;   // 0..1
            float  _Noise;      // 0..1

            v2f vert (appdata v){ v2f o; o.pos = UnityObjectToClipPos(v.vertex); o.uv = v.uv; return o; }

            // hash noise
            float hash21(float2 p){ p = frac(p*float2(123.34, 345.45)); p += dot(p, p+34.345); return frac(p.x*p.y); }

            fixed4 frag (v2f i) : SV_Target
            {
                float2 uv = i.uv;
                float4 prev = tex2D(_MainTex, uv);

                float2 d = uv - _BrushUVR.xy;
                float dist = length(d) / max(_BrushUVR.z, 1e-5);

                // borde suave
                float falloff = saturate(1.0 - smoothstep(_Hardness, 1.0, dist));

                // salpicado con ruido
                float n = hash21(floor(uv * 2048.0));
                float speckle = saturate(1.0 - step(0.9 + _Noise * 0.1, n));

                float a = falloff * (0.5 + 0.5*speckle);

                // Composición: color acumulativo + eleva alpha para “fuerza de mancha”
                float3 outCol = lerp(prev.rgb, _BrushColor.rgb, a);
                float outA = saturate(prev.a + a * 0.6);

                return float4(outCol, outA);
            }
            ENDHLSL
        }
    }
}
