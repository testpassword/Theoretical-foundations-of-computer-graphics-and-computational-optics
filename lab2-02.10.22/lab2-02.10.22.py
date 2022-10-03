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

obs000 = PlaneObserver()
obs000.res = 21, 31
obs000_n = ObserverNode(obs000)
obs000_n.Rotate(90, 0, 0)
obs000_n.Translate(0, -1, 0)

cm = ColorModel()
cm.SetSpectral()
scene = Scene()
scene.color_model = cm
for n in [rec_n, point_light_n, obs000_n]:
	scene.AddNode(n)
LoadScene(scene)
imaps = scene.IMapsParams()
imaps.SetObserverAsAccSource(obs000_n)
kernel = GetKernel()
kernel.CalculateIMaps()
