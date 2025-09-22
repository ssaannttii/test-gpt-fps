using UnityEngine;

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
        mr.material = new Material(Shader.Find("Universal Render Pipeline/Lit"));
        mr.material.SetColor("_BaseColor", paintColor * 1.2f);
    }
}
