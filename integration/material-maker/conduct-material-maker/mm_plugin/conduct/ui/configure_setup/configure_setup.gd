extends Popup

@onready var departmentOptions = $VBoxContainer/GridContainer/DepartmentOptions
@onready var assetOptions = $VBoxContainer/GridContainer/AssetOptions
@onready var sequenceOptions = $VBoxContainer/GridContainer/SequenceOptions
@onready var shotOptions = $VBoxContainer/GridContainer/ShotOptions
@onready var doneButton = $VBoxContainer/DoneButton

var summary = null
signal done(department, asset, sequence, shot)

# Called when the node enters the scene tree for the first time.
func _ready():
	departmentOptions.item_selected.connect(department_selected)
	assetOptions.item_selected.connect(asset_selected)
	sequenceOptions.item_selected.connect(sequence_selected)
	doneButton.button_down.connect(done_button_clicked)
	about_to_popup.connect(popup_begin)
	
	pass # Replace with function body.

func popup_begin():
	update_department_items()
	update_asset_items()
	update_sequence_items()

func done_button_clicked():
	var dept = ""
	var asset = ""
	var seq = ""
	var shot = ""
	
	
	dept = departmentOptions.get_item_text(departmentOptions.selected)
	asset = assetOptions.get_item_text(assetOptions.selected)
	if sequenceOptions.item_count > 0 && sequenceOptions.selected != 0: 
		seq = sequenceOptions.get_item_text(sequenceOptions.selected)
		
	if shotOptions.item_count > 0 && shotOptions.selected != 0:
		shot = shotOptions.get_item_text(shotOptions.selected)
	
	done.emit(
		dept,
		asset,
		seq,
		shot
	)
	hide()

func update_department_items():
	for i in range(departmentOptions.item_count):
		departmentOptions.remove_item(0)

	var id = 0
	for department in summary["departments"]:
		print(department)
		departmentOptions.add_item(department, id)
		id += 1

func update_asset_items():
	for i in range(assetOptions.item_count):
		assetOptions.remove_item(0)
		
	var dept = departmentOptions.get_item_text(departmentOptions.selected)
	var assets = []
	for asset in summary["assets"]:
		if dept in summary["assets"][asset]["departments"]:
			assets.append(asset)
	
	var id = 0	
	for asset in assets:
		assetOptions.add_item(asset, 0)

func update_sequence_items():
	for i in range(sequenceOptions.item_count):
		sequenceOptions.remove_item(0)
	
	sequenceOptions.add_item("-", 0)
	
	var id = 1
	for seq in summary["sequences"]:
		sequenceOptions.add_item(seq, id)
		id += 1
		
func update_shot_items():
	for i in range(shotOptions.item_count):
		shotOptions.remove_item(0)
		
	if sequenceOptions.selected == 0:
		return
		
	var seq = sequenceOptions.get_item_text(sequenceOptions.selected)
	
	shotOptions.add_item("-", 0)
	var id = 1
	for shot in summary["sequences"][seq]["shots"]:
		shotOptions.add_item(shot, id)
		id += 1

func department_selected(id):
	print(id)
	print(departmentOptions.get_item_text(id))
	update_asset_items()

func asset_selected(id):
	print(id)
	print(assetOptions.get_item_text(id))
	
func sequence_selected(id):
	print(id)
	print(sequenceOptions.get_item_text(id))
	update_shot_items()
