shape_lib = GetLibrary(Shape)
light_lib = GetLibrary(Light)
surf_lib = GetLibrary(SurfAttrs)


def build_observer_node(x: int = 0, y: int = 0, z: int = 0, angle: int = 30) -> Observer:
	obs = PlaneObserver()
	obs.res = 21, 31
	obs.phenom = ObserverData.LUM
	obs.x_side = 2000, 0, 0
	obs.y_side = 0, 3000, 0
	obs_n = ObserverNode(obs)
	obs_n.Rotate(90, 0, 0)
	obs_n.Translate(-1000, -1, -1500) # set center of a Observer to 0, 0, 0
	obs.dir = x, y, z
	obs.thresh_ang = angle
	return obs_n


rec = shape_lib.GetItem('Rectangle')
rec.size = 2000, 3000
rec.parts[0].surf_attrs.SetKd(0.66, BWSurfColor(0.91))
rec.sz = 0, 0, 1
rec_n = MeshNode(rec)
rec_n.Rotate(90, 0, 0)

point_light = light_lib.GetItem('Point')
point_light.radiometric = True
point_light.total_flux = 100
point_light.color = SpecLightColor([1]*41, range(380, 790, 10))
point_light_n = LightNode(point_light)
point_light_n.Translate(1000, -2000, 1500)
point_light_n.Rotate(90, 0, 0)

#(0, -1000, 0, 5), (0, -1000, -1000, 30), (0, -1000, -1000, 5)
obs_nodes = [build_observer_node(*it) for it in [(0, -1000, 0, 30)]]
cm = ColorModel([it * 10 + 370 for it in range(1, 42)])
cm.SetSpectral()
scene = Scene()
scene.color_model = cm
for n in ([rec_n, point_light_n] + obs_nodes):
	scene.AddNode(n)
LoadScene(scene)
imaps = scene.IMapsParams()
imaps.req_acc = 0.01
kernel = GetKernel()
for obs_n in obs_nodes:
	imaps.SetObserverAsAccSource(obs_n)
	#kernel.CalculateIMaps()
