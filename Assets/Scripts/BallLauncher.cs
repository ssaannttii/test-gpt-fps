using UnityEngine;

public class BallLauncher : MonoBehaviour
{
    public Transform cameraTransform;
    public KeyCode fireKey = KeyCode.Mouse1;
    public float speed = 28f;
    public float mass = 0.6f;
    public float radius = 0.15f;
    public PhysicMaterial physMat; // crea uno: fricción dinámica 0.2, restitución 0.65

    void Update()
    {
        if (Input.GetKeyDown(fireKey))
            Launch();
    }

    void Launch()
    {
        var go = GameObject.CreatePrimitive(PrimitiveType.Sphere);
        go.transform.position = cameraTransform.position + cameraTransform.forward * 0.4f;
        go.transform.localScale = Vector3.one * (radius * 2f);

        var col = go.GetComponent<SphereCollider>();
        if (physMat) col.material = physMat;

        var rb = go.AddComponent<Rigidbody>();
        rb.mass = mass;
        rb.interpolation = RigidbodyInterpolation.Interpolate;
        rb.collisionDetectionMode = CollisionDetectionMode.ContinuousDynamic;

        rb.AddForce(cameraTransform.forward * speed, ForceMode.VelocityChange);

        go.AddComponent<ImpactImpulse>(); // opcional: transfiere impulso extra al contacto
    }
}
