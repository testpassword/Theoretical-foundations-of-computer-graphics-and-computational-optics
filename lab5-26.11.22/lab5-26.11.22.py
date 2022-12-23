LIGHT_LIB = GetLibrary(Light)
FOCAL_LENGTH = 150
RESOLUTION = 600


def set_texture(part, color: tuple, mapping):
    part.surf_attrs.front_side.kd = 0.66
    part.surf_attrs.front_side.kd_color = RGBSurfColor(*color)
    modifier = TxtModifier()
    part.tma = 1
    modifier.mapping.texture = StdTexture('chess_bw.png', 1, 0)
    part.surf_attrs.front_side.kd_mod = modifier
    m3d = Mapping3D()
    m3d.type = mapping
    m3d.size = 100
    m3d.inher_mode = 2
    part.own_mapping3d = m3d


def build_geometry():
    cone = GetClass(Shape, 'Cone')(name = "cone", length = 200, radius1 = 10, radius2 = 50)
    set_texture(cone.parts[0], (0, 1, 0), 2)
    cone_node = MeshNode(cone)
    cone_transform = XYZTransform()
    cone_transform.pos = (-300, 0, 0)
    cone_node.tr = cone_transform
    box = GetClass(Shape, 'Box')(name = "box", centered = True, size = (150, 150, 200))
    for p in box.parts:
        set_texture(p, (1, 1, 0), 3)
    box_node = MeshNode(box)
    box_transform = XYZTransform()
    box_transform.pos = (-450, 100, 0)
    box_node.tr = box_transform
    sphere = GetClass(Shape, 'Sphere')(name = "sphere", radius = 150)
    set_texture(sphere.parts[0], (1, 0, 1), 1)
    sphere_node = MeshNode(sphere)
    sphere_transform = XYZTransform()
    sphere_transform.pos = (-700, -100, 0)
    sphere_node.tr = sphere_transform
    return [cone_node, box_node, sphere_node]


def build_parallel_light_node(azimut: int, tilt: int, rotate: int, offset: tuple):
    ln = LightNode(LIGHT_LIB.GetItem('Parallel'), name = 'parallelLight1') 
    lt = Transform()
    lt.azim = azimut
    lt.tilt = tilt
    lt.rot = rotate
    ln.tr = lt
    ln.Translate(*offset)
    return ln


def perform_calculations(lens_diam: float, scene, obs_nodes, run_pt: bool = True, run_imaps: bool = True):
    LoadScene(scene)
    kernel = GetKernel()
    if run_pt:
        pt_params = scene.PTRenderParams()
        pt_params.res = (RESOLUTION, RESOLUTION)
        kernel.PTRenderNotebook(scene.Notebook(), "d{}_pt.jpg".format(int(lens_diam)), OverwriteMode.OVERWRITE)
    if run_imaps:
        imaps = scene.IMapsParams()
        imaps.req_acc = 0.01
        imaps.time_limit = 60 * 60 * 4
        imaps.SetObserverAsAccSource(obs_nodes[0])  # function take one observer node as param, but all observer nodes from scene will be taken for calculations
        kernel.CalculateIMaps()



def calc_for_ideal_lens(lens_diam: float, run_pt: bool = True, run_imaps: bool = True):

    # need to calculate imaps
    def build_lens_observer_node(diam: float, focusing_dist: int, image_dist: int):
        lobs = LensObserver()    
        lobs.phenom = ObserverData.ILLUM
        lobs.pupil_diam = diam
        lobs.focal_length = FOCAL_LENGTH
        lobs.focusing_dist = focusing_dist
        lobs.view_angle = 70
        lobs.res = RESOLUTION, RESOLUTION
        lobs.image_dist = image_dist
        on = ObserverNode(lobs)
        on.name = "Lens Observer_{}".format(focusing_dist) 
        on.Translate(0, 0, 0)
        on.Rotate(-90, 0, -90)
        return on

    # need to perform path tracing
    def build_ideal_camera(diam: float, focusing_dist: int, image_dist: int):
        cam = Camera(70, focusing_dist)
        cam.is_lens = True
        cam.lens_locks = (CameraLensParamLock.FOCAL_LENGTH or CameraLensParamLock.TARGET_DISTANCE)
        cam.lens_type = IDEAL
        cam.pupil_diam = diam
        cam.SetLensParams(FOCAL_LENGTH, image_dist, focusing_dist)
        ideal_lens_300_transform = XYZTransform()
        ideal_lens_300_transform.pos = (0, 0, 0)
        ideal_lens_300_transform.x_rot_ang = -90
        ideal_lens_300_transform.y_rot_ang = 0
        ideal_lens_300_transform.z_rot_ang = -90
        cam.tr = ideal_lens_300_transform
        cam.name = "ideal_{}".format(focusing_dist)
        return cam

    obs_nodes = [
        build_lens_observer_node(*params) for params in [
            (lens_diam, 300, 300),
            (lens_diam, 375, 250),
            (lens_diam, 600, 200)
        ]
    ]
    light_nodes = [
        build_parallel_light_node(*params) for params in [
            (90, 45, 90, (-170, 0, 0)), 
            (135, 55, 135, (10, 0, 0)), 
            (225, 55, -135, (-250, 100, 100))
        ]
    ]
    cameras = [
        build_ideal_camera(*params) for params in [
            (lens_diam, 300, 300),
            (lens_diam, 375, 250),
            (lens_diam, 600, 200)
        ]
    ]
    scene = Scene()
    scene.CreateBackground().col_intensity = 0
    for node in light_nodes + obs_nodes + build_geometry():
        scene.AddNode(node)
    for camera in cameras:
        scene.Notebook().AddCamera(camera)
    perform_calculations(lens_diam, scene, obs_nodes, run_pt, run_imaps)


class RealLens:

    def __init__(self, lens_radius: float, scene_medium):
        self.scene_medium = scene_medium
        asurfs = [
            self.build_aspherical_surface(*params) for params in [
                ('asp0', (0, 0, 0), 160), 
                ('asp1', (0, 0, 30), -160)
            ]
        ]
        cylinder_lens = GetClass(Shape, 'Cylinder')(name = "Cylinder_Lens", radius = 37.5, length = 21.0886)
        cylinder_lens_part = cylinder_lens.parts[0]
        cylinder_lens_part.surf_attrs.front_side.kd = 0
        cylinder_lens_part.surf_attrs.front_side.kts = 1
        cylinder_lens_part.surf_attrs.front_side.gs = 100000
        cylinder_lens_part.surf_attrs.front_side.gm = 100000
        self.set_medium(cylinder_lens_part, "glass", 1.55, scene_medium)
        cylinder_lens_node = MeshNode(cylinder_lens,  name = "Cylinder_Lens")
        cylinder_lens_transform = XYZTransform()
        cylinder_lens_transform.pos = (0, 0, 15)
        cylinder_lens_node.tr = cylinder_lens_transform
        cone_lens = GetClass(Shape, 'Cone')(name = "Cone_Lens", radius1 = 37.5, radius2 = 415, length = 400)
        cone_lens.parts[0].surf_attrs.front_side.kd = 0
        cone_lens.parts[0].surf_attrs.front_side.kts = 0
        cone_lens.parts[0].surf_attrs.front_side.gs = 100000
        cone_lens.parts[0].surf_attrs.front_side.gm = 100000
        self.set_env_medium(cone_lens.parts[0], scene_medium)
        cone_node_lens = MeshNode(cone_lens,  name = "Cone_Lens")
        cone_transform = XYZTransform()
        cone_transform.pos = (0, 0, 204.4557)
        cone_node_lens.tr = cone_transform
        RingClass = GetClass(Shape, 'Ring')
        ring_lens = RingClass(name = "Ring_Lens", org = (0, 0, 0),  centered = True, radius1 = lens_radius, radius2 = 62)
        ring_lens.parts[0].surf_attrs.front_side.kd = 0
        ring_lens.parts[0].surf_attrs.front_side.kts = 0
        ring_lens.parts[0].surf_attrs.front_side.gs = 100000
        ring_lens.parts[0].surf_attrs.front_side.gm = 100000
        self.set_env_medium(ring_lens.parts[0], scene_medium)
        ring_node_lens = MeshNode(ring_lens, name = "Ring_lens")
        ring_transform = XYZTransform()
        ring_transform.pos = (0, 0, 30)
        ring_node_lens.tr = ring_transform
        lens_wrapper = Node(name = 'lens_wrapper')
        lens_wrapper.AddNode(cone_node_lens)
        lens_wrapper.AddNode(ring_node_lens)
        lens_wrapper.AddNode(cylinder_lens_node)
        for asp in asurfs:
            lens_wrapper.AddNode(asp)
        p_obs = [
            self.build_plane_observer_node(*params) for params in [
                ((-210, -210, 314.4548), 420.1038, 300),
                ((-175, -175, 264.4548), 350.1038, 375),
                ((-140, -140, 214.4548), 280.0830, 600),
            ]
        ]
        for obs in p_obs:
            lens_wrapper.AddNode(obs)
        lens_wrapper_transform = XYZTransform()
        lens_wrapper_transform.pos = (0, 0, 0)
        lens_wrapper_transform.x_rot_ang = -90
        lens_wrapper_transform.y_rot_ang = 0
        lens_wrapper_transform.z_rot_ang = -90
        lens_wrapper.tr = lens_wrapper_transform
        self.observers = p_obs
        self.lens_wrapper = lens_wrapper

    def set_medium(self, part, name_inside, inside_refr_ind, env_medium):
        part.back_medium = SimpleMedium(name = name_inside, refr_ind = inside_refr_ind)
        part.front_medium = env_medium

    def set_env_medium(self, part, env_medium):
        part.back_medium = env_medium
        part.front_medium = env_medium

    def build_aspherical_surface(self, name: str, position: tuple, rad: int):
        asurf = GetClass(Shape, 'Aspherical Surface')(name = name, h = 37.5, h0 = 0, r = rad)
        p = asurf.parts[0]
        p.surf_attrs.front_side.kd = 0
        p.surf_attrs.front_side.kts = 1
        p.surf_attrs.front_side.gs = 100000
        p.surf_attrs.front_side.gm = 100000
        self.set_medium(p, "glass", 1.55, self.scene_medium)
        asurfn = MeshNode(asurf, name = "AsphericalSurface1_Lens")
        asurnt = XYZTransform()
        asurnt.pos = position
        asurfn.tr  = asurnt
        return asurfn

    def build_plane_observer_node(self, origin: tuple, size: float, focusing_dist: int):
        pobs = PlaneObserver()    
        pobs.phenom = 1  
        pobs.res = RESOLUTION, RESOLUTION
        pobs.thresh_ang = 70                                            
        pobs.org = origin
        pobs.x_side = (size, 0, 0)
        pobs.y_side = (0, size, 0)
        pobs.dir = (0, 0, 100)
        pobs.phenom = ObserverData.ILLUM
        onode = ObserverNode(pobs)
        onode.name = "Plane Observer_{}".format(focusing_dist)
        onode.color = (255, 255, 0)
        return onode


def calc_for_real_leas(lens_diam: float, run_pt: bool = True, run_imaps: bool = True):

    def build_real_camera(diam: float, focusing_dist: int, image_dist: int):
        cam = Camera(70, focusing_dist)
        cam.is_lens = True
        cam.lens_locks = (CameraLensParamLock.FOCAL_LENGTH or CameraLensParamLock.TARGET_DISTANCE)
        cam.lens_type = REAL
        cam.lens_thickness = 15
        cam.pupil_diam = diam
        cam.image_dist = image_dist
        cam.object_dist = focusing_dist
        cam.focal_length = FOCAL_LENGTH
        rlt = XYZTransform()
        rlt.pos = (0, 0, 0)
        rlt.x_rot_ang = -90
        rlt.y_rot_ang = 0
        rlt.z_rot_ang = -90
        cam.tr = rlt
        cam.name = "real_{}".format(focusing_dist)
        return cam
    
    light_nodes = [
        build_parallel_light_node(*params) for params in [
            (90, 45, 90, (-170, 0, 0)), 
            (135, 55, 135, (10, 0, 0)), 
            (225, 55, -135, (-250, 100, 100))
        ]
    ]
    cameras = [
        build_real_camera(*params) for params in [
            (lens_diam, 300, 300),
            (lens_diam, 375, 250),
            (lens_diam, 600, 200),
        ]
    ]
    scene = Scene()
    scene.CreateBackground().col_intensity = 0
    real_lens = RealLens(lens_diam / 2, scene.GetMedium("env"))
    for node in light_nodes + build_geometry() + [real_lens.lens_wrapper]:
        scene.AddNode(node)
    for camera in cameras:
        scene.Notebook().AddCamera(camera)
        LoadScene(scene)
    perform_calculations(lens_diam, scene, real_lens.observers, run_pt, run_imaps)


for diam in [18.75, 37.5, 75]:
    calc_for_ideal_lens(diam, True, False)
    calc_for_real_leas(diam, True, False)