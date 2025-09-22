using UnityEngine;
using UnityEngine.Rendering;

public class PaintGun : MonoBehaviour
{
    public Transform cameraTransform;
    public float fireRate = 10f;
    public float muzzleVelocity = 40f;
    public float spread = 0.5f;
    public Color paintColor = new Color(0.1f, 0.6f, 1f, 1f);
    public float paintRadius = 0.06f; // metros aproximados

    float nextFire;

    void Update()
    {
        if (Input.GetMouseButton(0) && Time.time >= nextFire)
        {
            nextFire = Time.time + 1f / fireRate;
            Fire();
        }
    }

    static readonly int BaseColorId = Shader.PropertyToID("_BaseColor");
    static readonly int ColorId = Shader.PropertyToID("_Color");
    static Shader cachedProjectileShader;

    void Fire()
    {
        var dir = cameraTransform.forward;
        dir = Quaternion.Euler(Random.Range(-spread, spread), Random.Range(-spread, spread), 0) * dir;

        var go = GameObject.CreatePrimitive(PrimitiveType.Sphere);
        go.transform.position = cameraTransform.position + cameraTransform.forward * 0.2f;
        go.transform.localScale = Vector3.one * 0.06f;
        var rb = go.AddComponent<Rigidbody>();
        rb.collisionDetectionMode = CollisionDetectionMode.ContinuousDynamic;
        rb.mass = 0.02f; // paintball
        var proj = go.AddComponent<PaintProjectile>();
        proj.paintColor = paintColor;
        proj.paintRadius = paintRadius;
        rb.AddForce(dir * muzzleVelocity, ForceMode.VelocityChange);

        // visual
        var mr = go.GetComponent<MeshRenderer>();
        var shader = GetProjectileShader();
        Material projectileMaterial = null;

        if (shader != null)
        {
            projectileMaterial = new Material(shader);
            SetProjectileColor(projectileMaterial, paintColor * 1.2f);
            mr.material = projectileMaterial;
        }
        else
        {
            Debug.LogWarning("PaintGun: Unable to find URP Lit shader, falling back to default material color.", this);
            projectileMaterial = mr.material;
            SetProjectileColor(projectileMaterial, paintColor * 1.2f);
        }
    }

    static Shader GetProjectileShader()
    {
        if (cachedProjectileShader != null)
            return cachedProjectileShader;

        var currentPipeline = GraphicsSettings.currentRenderPipeline;
        if (currentPipeline != null)
            cachedProjectileShader = currentPipeline.defaultShader;

        if (cachedProjectileShader == null)
            cachedProjectileShader = Shader.Find("Universal Render Pipeline/Lit");

        return cachedProjectileShader;
    }

    static void SetProjectileColor(Material material, Color color)
    {
        if (material == null)
            return;

        if (material.HasProperty(BaseColorId))
            material.SetColor(BaseColorId, color);
        else if (material.HasProperty(ColorId))
            material.SetColor(ColorId, color);
        else
            material.color = color;
    }
}
