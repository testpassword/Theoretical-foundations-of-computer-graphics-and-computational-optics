import re

def get_name_and_ext(filepath: str) -> tuple:
	tokens = filepath.rsplit('.', 1)
	return tokens[0], tokens[1]


def nit_to_txt(filepath: str):
	PostProcessor(filepath).SaveToText(
		get_name_and_ext(filepath)[0] + '.txt', 
		format=PPOutputFormat.RGB, 
		overwrite=OverwriteMode.OVERWRITE
	)


def txt_to_nit(filepath: str):
	w = h = line_num = 0
	color_channel_range = (0, 0) # number of lines with start and finish of color block
	r, g, b = [[]] * 3
	processed_channel = None
	color_is_init = False
	combine_spaces = re.compile(r"\s+")
	with open(filepath, 'r') as file:
		for l in file:
			line_num += 1

			def processed_color_channel(ch_arr):
				color_channel = [float(it) for it in combine_spaces.sub(" ", l).strip().split(' ')[2:]] # [2:] means remove index and y/x
				for i, ch in enumerate(color_channel):
					r[line_num - color_channel_range[0]][i] = ch

			def extract_resolution(direction: str) -> int:
				return int(l.replace(direction, '').replace('pix', '').strip())

			def set_processed_color_channel(color_ch_arr):
				# start is a line with color_name + header, end is a line with color_name header + + height - empty_string
				color_channel_range = (line_num + 3, line_num + 3 + h - 1)
				processed_channel = color_ch_arr

			if 'Horizontal' in l:
				w = extract_resolution('Horizontal')
			if 'Vertical' in l:
				h = extract_resolution('Vertical')
			if w and h and not color_is_init:
				r, g, b = [[[None] * w] * h] * 3
				color_is_init = True
			if 'red' in l and 'index' in l:
				set_processed_color_channel(r)
			if 'green' in l and 'index' in l:
				set_processed_color_channel(g)
			if 'blue' in l and 'index' in l:
				set_processed_color_channel(b)
			if line_num >= color_channel_range[0] and line_num <= color_channel_range[1]:
				processed_color_channel(processed_channel)
		print("r is")
		print(r)
		PostProcessor(PPDataUnits.LUMINANCE, [], r, g, b).SaveToHDR(
			get_name_and_ext(filepath)[0] + '-OUT.nit',
			overwrite=OverwriteMode.OVERWRITE
		)
			


#nit_to_txt('Debug\\debug.nit')
txt_to_nit('Debug\\debug.txt')
