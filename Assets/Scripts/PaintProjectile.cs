using UnityEngine;

[RequireComponent(typeof(Rigidbody), typeof(Collider))]
public class PaintProjectile : MonoBehaviour
{
    public Color paintColor = Color.cyan;
    public float paintRadius = 0.06f;
    public float splashCount = 12f; // nº de “gotas” por impacto
    public float impulse = 60f;     // fuerza radial

    void OnCollisionEnter(Collision c)
    {
        // Impulso radial a rigidbodies cercanos
        var hits = Physics.OverlapSphere(transform.position, paintRadius * 8f);
        foreach (var h in hits)
        {
            var rb = h.attachedRigidbody;
            if (rb != null)
            {
                rb.AddExplosionForce(impulse, transform.position, paintRadius * 8f, 0.1f, ForceMode.Impulse);
            }
        }

        // “Salpicadura”: múltiples blits en torno al punto de impacto
        if (c.contactCount > 0)
        {
            var contact = c.GetContact(0);
            TryPaint(contact, 1f); // mancha central

            for (int i = 0; i < splashCount; i++)
            {
                if (!Physics.Raycast(contact.point + contact.normal * 0.01f,
                                     Random.insideUnitSphere, out RaycastHit hit, paintRadius * 4f))
                    continue;
                TryPaint(hit, Random.Range(0.3f, 0.8f));
            }
        }

        Destroy(gameObject);
    }

    void TryPaint(ContactPoint cp, float scale)
    {
        var col = cp.otherCollider;
        if (col.TryGetComponent<Paintable>(out var paintable))
        {
            paintable.PaintAtWorld(cp.point, cp.normal, paintColor, paintRadius * scale);
        }
    }

    void TryPaint(RaycastHit hit, float scale)
    {
        if (hit.collider.TryGetComponent<Paintable>(out var paintable))
        {
            paintable.PaintAtWorld(hit.point, hit.normal, paintColor, paintRadius * scale);
        }
    }
}
