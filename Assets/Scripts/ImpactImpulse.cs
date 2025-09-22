using UnityEngine;

public class ImpactImpulse : MonoBehaviour
{
    public float extraImpulse = 2.5f;

    void OnCollisionEnter(Collision c)
    {
        var rb = c.rigidbody;
        if (rb != null) rb.AddForceAtPosition(-c.relativeVelocity.normalized * extraImpulse, c.contacts[0].point, ForceMode.Impulse);
    }
}
