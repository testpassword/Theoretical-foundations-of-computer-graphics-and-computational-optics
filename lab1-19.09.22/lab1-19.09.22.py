import sys		
import random
import numpy as np
import uuid

NAME = uuid.uuid4().hex


def get_mode() -> list:
	return ['Plane', 'Point', 'Box']  #default


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


def create_scene(*nodes: Node) -> Scene:
	scene = Scene()
	for n in nodes:
		scene.AddNode(n)
	return scene


def rand_translate(node: Node) -> Node:
	# Node.Translate is SM, but Transformation from GUI is M
	node.Translate(*np.full(3, random.randint(0, 500))) #[0; 500] range for visibility in default camera
	return node


# todo: gonio observer, gonio light

obs, light, shape = [it[0].GetItem(it[1]) for it in zip([ObserverLibrary, GetLibrary(Light), GetLibrary(Shape)], get_mode())]
light.color = SpecLightColor(RGBLightColor(*np.random.rand(3)))
light.total_flux = random.randint(500, 1000)  #less than 500lm is dim
#create a large Observer to catch everything
obs.res = 128, 128
obs.org = -1000, -1000, 700
obs.x_side = 2000, 0, 0
obs.y_side = 0, 2000, 0
obs_n, light_n, shape_n = [it[0](it[1]) for it in zip([ObserverNode, LightNode, MeshNode], [obs, light, shape])]
for n in [light_n, shape_n]:
	rand_translate(n)
sc = create_scene(obs_n, light_n, shape_n)
LoadScene(sc)
render_params = sc.RenderParams()
render_params.res = 128, 128
imaps_params = sc.IMapsParams()
imaps_params.SetObserverAsAccSource(obs_n)
notebook = sc.Notebook()
kernel = GetKernel()
kernel.CalculateIMaps()
kernel.RenderNotebook(notebook, '{}.jpg'.format(NAME), OverwriteMode.OVERWRITE)
kernel.Render()
sc.Save(NAME, OverwriteMode.OVERWRITE)