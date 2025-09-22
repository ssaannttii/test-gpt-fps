using UnityEngine;
using UnityEngine.Rendering;

[RequireComponent(typeof(Renderer))]
public class Paintable : MonoBehaviour
{
    public int rtSize = 1024;
    public RenderTexture paintRT;
    static Material blitMat; // asigna M_PaintBlit desde Resources.FindObjectsOfTypeAll si quieres

    Renderer rend;
    MaterialPropertyBlock mpb;

    void Awake()
    {
        rend = GetComponent<Renderer>();
        mpb = new MaterialPropertyBlock();

        // RenderTexture donde “acumulamos” pintura
        paintRT = new RenderTexture(rtSize, rtSize, 0, RenderTextureFormat.ARGB32);
        paintRT.enableRandomWrite = false;
        paintRT.wrapMode = TextureWrapMode.Clamp;
        paintRT.filterMode = FilterMode.Bilinear;
        paintRT.Create();

        rend.GetPropertyBlock(mpb);
        mpb.SetTexture("_PaintTex", paintRT);
        rend.SetPropertyBlock(mpb);

        // material del blit
        if (blitMat == null)
        {
            var mats = Resources.FindObjectsOfTypeAll<Material>();
            foreach (var m in mats)
                if (m && m.shader && m.shader.name == "Hidden/RT_DrawCircle") { blitMat = m; break; }
            if (blitMat == null)
                blitMat = new Material(Shader.Find("Hidden/RT_DrawCircle"));
        }
    }

    public void PaintAtWorld(Vector3 worldPos, Vector3 worldNormal, Color color, float radiusMeters)
    {
        if (!TryGetUV(worldPos, out Vector2 uv)) return;

        // Convierte radio en píxeles aproximados (usa bounds como referencia)
        float approxMetersToUV = 1f / Mathf.Max(transform.lossyScale.x, transform.lossyScale.z);
        float rUV = radiusMeters * approxMetersToUV;

        // Parámetros para el shader de blit
        blitMat.SetVector("_BrushUVR", new Vector4(uv.x, uv.y, rUV, 0));
        blitMat.SetColor("_BrushColor", color);
        blitMat.SetFloat("_Hardness", 0.7f);  // borde suave
        blitMat.SetFloat("_Noise", 0.35f);    // salpicado

        // Blit aditivo (sobre RT)
        var tmp = RenderTexture.GetTemporary(paintRT.descriptor);
        Graphics.Blit(paintRT, tmp);
        Graphics.Blit(tmp, paintRT, blitMat);
        RenderTexture.ReleaseTemporary(tmp);
    }

    bool TryGetUV(Vector3 worldPos, out Vector2 uv)
    {
        uv = Vector2.zero;
        var ray = new Ray(worldPos + (worldPos - transform.position).normalized * 0.01f,
                          (transform.position - worldPos).normalized); // pequeño rayo inverso

        if (GetComponent<Collider>().Raycast(ray, out RaycastHit hit, 1f))
        {
            uv = hit.textureCoord;
            return true;
        }
        return false;
    }

    void OnDestroy()
    {
        if (paintRT) paintRT.Release();
    }
}
