import sys		
import random
import numpy as np
import uuid
from collections import namedtuple


#the is no ObserverLibrary in API, so make something similar
class ObserverLibrary:
	@staticmethod
	def GetItem(type_of_observer: str) -> Observer:
		try:
			return {
				'Plane': PlaneObserver,
				'Gonio': GonioObserver
			}[type_of_observer]()
		except KeyError:
			return None


# create scene with nodes at once
def create_scene(*nodes: Node) -> Scene:
	scene = Scene()
	for n in nodes:
		scene.AddNode(n)
	return scene


# random translate node
def rand_translate(node: Node) -> Node:
	# Node.Translate is centimeters, but Transformation from GUI is meters
	node.Translate(*np.full(3, random.randint(0, 500))) #[0; 500] range for visibility in default camera
	return node


NAME = uuid.uuid4().hex
obs_lib = ObserverLibrary
light_lib = GetLibrary(Light)
shape_lib = GetLibrary(Shape)
obs = ObserverLibrary.GetItem('Plane')
point_light = light_lib.GetItem('Point')
point_light.gonio = GetLibrary(Gonio).GetFolder('Standard').GetItem('lambertian')
rectangle_light = light_lib.GetItem('Rectangle')
for l in [point_light, rectangle_light]:
	l.color = SpecLightColor(RGBLightColor(*np.random.rand(3)))
	l.total_flux = random.randint(500, 1000)  #less than 500lm is dim
box = shape_lib.GetItem('Box')
sphere = shape_lib.GetItem('Sphere')
sphere.radius = 100
#create a large Observer to catch everything
obs.res = 128, 128
obs.org = -1000, -1000, 700
obs.x_side = 2000, 0, 0
obs.y_side = 0, 2000, 0
obs.dir = 0, 0, -1000
obs_n, point_light_n, rectangle_light_n, box_n, sphere_n = [
	it[0](it[1]) for it in zip(
		[ObserverNode, LightNode, LightNode, MeshNode, MeshNode], 
		[obs, point_light, rectangle_light, box, sphere])
]
point_light_n.Translate(500, 500, 750)
rectangle_light_n.Translate(-500, 500, 750)
for s in [box_n, sphere_n]:
	rand_translate(s)
sc = create_scene(obs_n, point_light_n, rectangle_light_n, box_n, sphere_n)
LoadScene(sc)
render_params = sc.RenderParams()
render_params.res = 256, 256
imaps_params = sc.IMapsParams()
imaps_params.SetObserverAsAccSource(obs_n)
notebook = sc.Notebook()
kernel = GetKernel()
kernel.CalculateIMaps()
kernel.RenderNotebook(notebook, '{}.jpg'.format(NAME), OverwriteMode.OVERWRITE)
kernel.Render()
sc.Save(NAME, OverwriteMode.OVERWRITE)
