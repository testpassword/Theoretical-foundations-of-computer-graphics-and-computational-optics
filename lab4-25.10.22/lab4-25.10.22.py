import re


def get_name_and_ext(filepath: str) -> tuple:
	tokens = filepath.rsplit('.', 1)
	return tokens[0], tokens[1]


def create_3d_arr(x, y, z) -> list:
	arr = [None] * x
	for i in range(x):
		arr[i] = [None] * y
		for j in range(y):
			arr[i][j] = [None] * z
	return arr


def nit_to_txt(filepath: str):
	PostProcessor(filepath).SaveToText(
		get_name_and_ext(filepath)[0] + '.txt', 
		format=PPOutputFormat.RGB, 
		overwrite=OverwriteMode.OVERWRITE
	)


def txt_to_nit(filepath: str):
	w = h = line_num = 0
	color_channel_range = 0, 0  # number of lines with start and finish of color channel in file
	processed_channel_index = None  # 0 for red, 1 for green, 2 for blue
	rgb = []
	color_is_init = False
	combine_spaces = re.compile(r"\s+")
	with open(filepath, 'r', encoding='utf-8') as file:
		for l in file:
			line_num += 1

			def processed_color_channel():
				color_channel = [float(it) for it in combine_spaces.sub(" ", l).strip().split(' ')[2:]] # [2:] means remove index and y/x
				for i, ch in enumerate(color_channel):
					rgb[processed_channel_index][line_num - color_channel_range[0]][i] = ch

			def extract_resolution(direction: str) -> int:
				return int(l.replace(direction, '').replace('pix', '').strip())

			def set_processed_color_channel(color_ch_index):
				nonlocal color_channel_range, processed_channel_index
				# start is a line with color_name + header, end is a line with color_name header + + height - empty_string
				color_channel_range = line_num + 3, line_num + 3 + h - 1
				processed_channel_index = color_ch_index

			if 'Horizontal' in l:
				w = extract_resolution('Horizontal')
			if 'Vertical' in l:
				h = extract_resolution('Vertical')
			if w and h and bool(rgb) is False:
				rgb = create_3d_arr(3, h, w)
			for i, it in enumerate(['red', 'green', 'blue']):
				if it in l and 'index' in l:
					set_processed_color_channel(i)
			if line_num >= color_channel_range[0] and line_num <= color_channel_range[1]:
				processed_color_channel()
		PostProcessor(PPDataUnits.LUMINANCE, [], *rgb).SaveToHDR(
			get_name_and_ext(filepath)[0] + '-OUT.nit',
			overwrite=OverwriteMode.OVERWRITE
		)
			

debug = 'Debug\\debug'
release = 'illuminance'
workdir = 'C:\\Users\\kulba\\Desktop\\Theoretical-foundations-of-computer-graphics-and-computational-optics\\lab4-25.10.22\\'
nit_to_txt(workdir + release + '.nit')
txt_to_nit(workdir + release + '.txt')
