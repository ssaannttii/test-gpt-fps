using UnityEngine;

public class GameInstaller : MonoBehaviour
{
    [Header("Scene")]
    public Material paintableMaterial;   // Asigna un material que use URP_PaintableShader
    public PhysicMaterial realisticPhysMat;

    void Awake()
    {
        // Luz
        var lightGO = new GameObject("Directional Light");
        var light = lightGO.AddComponent<Light>();
        light.type = LightType.Directional;
        light.intensity = 1.2f;
        light.transform.rotation = Quaternion.Euler(50, -30, 0);

        // Suelo
        var floor = GameObject.CreatePrimitive(PrimitiveType.Plane);
        floor.name = "Floor";
        floor.transform.localScale = new Vector3(4, 1, 4);
        floor.GetComponent<MeshRenderer>().material = MakePaintableMat();
        floor.AddComponent<Paintable>();

        // Paredes
        CreateWall(new Vector3(0, 2.5f, 20), new Vector3(40, 5, 1));
        CreateWall(new Vector3(0, 2.5f, -20), new Vector3(40, 5, 1));
        CreateWall(new Vector3(20, 2.5f, 0), new Vector3(1, 5, 40));
        CreateWall(new Vector3(-20, 2.5f, 0), new Vector3(1, 5, 40));

        // Objetos pintables sueltos
        for (int i = 0; i < 6; i++)
        {
            var cube = GameObject.CreatePrimitive(PrimitiveType.Cube);
            cube.transform.position = new Vector3(Random.Range(-8, 8), 0.5f, Random.Range(5, 15));
            cube.GetComponent<MeshRenderer>().material = MakePaintableMat();
            cube.AddComponent<Paintable>();

            // algunos con rigidbody para que reciban impulsos
            if (i % 2 == 0)
            {
                var rb = cube.AddComponent<Rigidbody>();
                rb.mass = 3f;
                rb.interpolation = RigidbodyInterpolation.Interpolate;
                if (realisticPhysMat) cube.GetComponent<Collider>().material = realisticPhysMat;
            }
        }

        // FPS Controller
        var player = new GameObject("Player");
        var cam = new GameObject("Main Camera");
        cam.tag = "MainCamera";
        cam.AddComponent<Camera>();
        cam.transform.SetParent(player.transform);
        cam.transform.localPosition = new Vector3(0, 1.6f, 0);

        var ctrl = player.AddComponent<MinimalFPSController>();
        ctrl.cameraTransform = cam.transform;

        // PaintGun
        var gun = player.AddComponent<PaintGun>();
        gun.cameraTransform = cam.transform;

        // Ball Launcher
        var ball = player.AddComponent<BallLauncher>();
        ball.cameraTransform = cam.transform;

        player.transform.position = new Vector3(0, 1.8f, -10);
    }

    void CreateWall(Vector3 center, Vector3 size)
    {
        var wall = GameObject.CreatePrimitive(PrimitiveType.Cube);
        wall.name = "Wall";
        wall.transform.position = center;
        wall.transform.localScale = size;
        wall.GetComponent<MeshRenderer>().material = MakePaintableMat();
        wall.AddComponent<Paintable>();
    }

    Material MakePaintableMat()
    {
        var mat = new Material(Shader.Find("Universal Render Pipeline/URP_Paintable"));
        // base color gris claro
        mat.SetColor("_BaseColor", new Color(0.7f, 0.7f, 0.7f, 1f));
        return mat;
    }
}
