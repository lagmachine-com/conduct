extends Node
class_name Vessel

var conduct_exe: String
func set_manifest(manifest_path: String):
	var dir = manifest_path.get_base_dir()
	conduct_exe = dir + "/" + conduct_exe_name()

func conduct_exe_name() -> String:
	conduct_exe = "conduct"
	if OS.get_name() == "Windows":
		conduct_exe += ".exe"
	return conduct_exe
	
func find_conduct_exe(current_file: String) -> bool:
	var path = current_file.get_base_dir()
	
	while path != "/" and path != "":
		var exe = path + "/" + conduct_exe_name()
		print(exe)
		var exists = FileAccess.file_exists(exe)
		print(exists)
		
		if exists:
			conduct_exe = exe
			return true
			
		path = path.get_base_dir()

	return false

func run_process(args: PackedStringArray) -> Dictionary:
	var output = []
	print("Executing")
	print(args)
	var exit_code = OS.execute(conduct_exe, args, output, false, false)
	print(output[0])
	var result = JSON.parse_string(output[0])
	return result

func get_summary():
	return run_process(["--command", "summary"])

func setup(file_format: String):
	var args = ["dialog", "create_setup", "--", "--file-format", file_format]
	return run_process(args)

func export(department, format, asset, element = "",  shot = ""):
	var args = ["export"]

	if department != "" and department != null:
		args.append("--department")
		args.append(department)

	if asset != "" and asset != null:
		args.append("--asset")
		args.append(asset)

	if shot != "" and shot != null:
		args.append("--shot")
		args.append(shot)
		
	if element != "" and element != null:
		args.append("--element")
		args.append(element)
		
	if format != "" and format != null:
		args.append("--file-format")
		args.append(format)
	
	args.append("--from")
	args.append("material_maker")
	
	return run_process(args)
