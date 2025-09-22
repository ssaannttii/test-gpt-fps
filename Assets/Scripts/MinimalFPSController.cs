using UnityEngine;

[RequireComponent(typeof(CharacterController))]
public class MinimalFPSController : MonoBehaviour
{
    public Transform cameraTransform;
    public float moveSpeed = 6f;
    public float mouseSensitivity = 2.2f;
    public float gravity = -9.81f;

    CharacterController cc;
    float pitch = 0f;
    Vector3 velocity;

    void Start()
    {
        cc = GetComponent<CharacterController>();
        cc.height = 1.8f;
        cc.center = new Vector3(0, 0.9f, 0);
        Cursor.lockState = CursorLockMode.Locked;
    }

    void Update()
    {
        // Mouse look
        float mx = Input.GetAxis("Mouse X") * mouseSensitivity;
        float my = Input.GetAxis("Mouse Y") * mouseSensitivity;
        transform.Rotate(0, mx, 0);
        pitch = Mathf.Clamp(pitch - my, -85, 85);
        cameraTransform.localEulerAngles = new Vector3(pitch, 0, 0);

        // Move
        float h = Input.GetAxis("Horizontal");
        float v = Input.GetAxis("Vertical");
        Vector3 dir = (transform.right * h + transform.forward * v);
        cc.Move(dir * moveSpeed * Time.deltaTime);

        // Gravity
        if (cc.isGrounded && velocity.y < 0) velocity.y = -2f;
        velocity.y += gravity * Time.deltaTime;
        cc.Move(velocity * Time.deltaTime);
    }
}
