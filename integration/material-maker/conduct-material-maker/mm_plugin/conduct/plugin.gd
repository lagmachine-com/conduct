extends Node
class_name VesselIntegration

var menu_entry = preload("res://mm_plugin/conduct/ui/top_bar_menu.tscn")
@onready var conduct_api = $conduct_api

# Called when the node enters the scene tree for the first time.
func _ready():
	print("Hello from plugin world!")
	print("Plugin init was called!")


func on_ui_loaded():
	var globals = get_node("/root/mm_globals")
	print(globals)
	
	var menu = globals.main_window.get_node("VBoxContainer/TopBar")
	var hbox = HBoxContainer.new()
	menu.add_child(hbox)
	var entry = menu_entry.instantiate()
	print("Setting entry")
	entry.plugin = self
	
	print("Adding child")
	hbox.add_child(entry)
	
		
	var manager = get_node("/root/MainWindow/NodeLibraryManager")
	print(manager)
	
	var LIBRARY = load("res://material_maker/tools/library_manager/library.gd")
	
	var data = load("res://mm_plugin/conduct/node_library/node_library.json")
	print(data)
	data = JSON.stringify(data.data)
	var lib = LIBRARY.new()
	lib.load_library("", true, data)
	
	manager.add_child(lib)
	manager.emit_signal("libraries_changed")
	lib.get_sections()
	print("Loaded library!")
