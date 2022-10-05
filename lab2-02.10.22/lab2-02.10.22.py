shape_lib = GetLibrary(Shape)
light_lib = GetLibrary(Light)

rec = shape_lib.GetItem('Rectangle')
rec.size = 2000, 3000
rec.sz = 0, 0, 1
rec_n = MeshNode(rec)
rec_n.Rotate(90, 0, 0)

point_light = light_lib.GetItem('Point')
point_light.radiometric = True
point_light.total_flux = 100
point_light.color = SpecLightColor('spectral_pink.spd')
point_light_n = LightNode(point_light)
point_light_n.Translate(1000, -2000, 1500)
point_light_n.Rotate(90, 0, 0)

obs = PlaneObserver()
obs.res = 21, 31
obs.phenom = ObserverData.ILLUM
obs.x_side = 2000, 0, 0
obs.y_side = 0, 3000, 0
obs.dir = 0, 0, - 100
obs_n = ObserverNode(obs)
obs_n.Rotate(90, 0, 0)
obs_n.Translate(-1000, -1, -1500) # set center of a Observer to 0, 0, 0

cm = ColorModel([it * 10 + 370 for it in range(1, 42)])
cm.SetSpectral()
scene = Scene()
scene.color_model = cm
for n in [rec_n, point_light_n, obs_n]:
	scene.AddNode(n)
LoadScene(scene)
imaps = scene.IMapsParams()
imaps.req_acc = 0.01
imaps.SetObserverAsAccSource(obs_n)
kernel = GetKernel()
kernel.CalculateIMaps()
