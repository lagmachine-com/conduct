extends Node
class_name ConductIntegration
@onready var conduct_api = $conduct_api

@onready var mm = get_node("/root/mm_plugins/com_lagmachine_conduct")

func get_id() -> String:
	return "com_lagmachine_conduct"

func _ready() -> void:
	mm.on_ui_loaded.connect(on_ui_loaded)

func on_ui_loaded():
	var globals = get_node("/root/mm_globals")
	print(globals)
	print("MM Api: ")
	print(mm)
	var menu = globals.main_window.get_node("VBoxContainer/TopBar")
	var hbox = HBoxContainer.new()

	menu.add_child(hbox)
	var menu_entry = mm.load("res://com_lagmachine_conduct/ui/top_bar_menu.tscn")
	var entry = menu_entry.instantiate()
	print("Setting entry")
	entry.plugin = self
	
	print("Adding child")
	hbox.add_child(entry)

	var manager = get_node("/root/MainWindow/NodeLibraryManager")
	print(manager)
	
	var LIBRARY = mm.load("res://material_maker/tools/library_manager/library.gd")
	
	var data = mm.load("res://com_lagmachine_conduct/node_library/node_library.json")
	print(data)
	data = JSON.stringify(data.data)
	var lib = LIBRARY.new()
	lib.load_library("", true, data)
	
	manager.add_child(lib)
	manager.emit_signal("libraries_changed")
	lib.get_sections()
	print("Loaded library!")
