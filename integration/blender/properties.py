import bpy
from bpy.app.handlers import persistent
import random
import time

data_cache = None

formats = []

formats.append("a")
formats.append("b")
formats.append("c")
formats.append("d")
formats.append("e")

random.seed(time.process_time_ns())
random.shuffle(formats)

def get_formats():
    return formats

def target_file_format_callback(self, context):
    from . import utils

    file_formats = []
    for format in get_formats():
        file_formats.append((format, format, format))

    return file_formats


def target_file_format_getter(self):
    items = get_formats()
    try:
        return items.index(self.format_str)
    except:
        return 0

def target_file_format_setter(self, value):
    items = get_formats()
    self.format_str = items[value]
    pass

class ExportObject(bpy.types.PropertyGroup):
    Action : bpy.props.PointerProperty(type=bpy.types.Action)
    Armature : bpy.props.PointerProperty(type=bpy.types.Armature)
    Brush : bpy.props.PointerProperty(type=bpy.types.Brush)
    Camera : bpy.props.PointerProperty(type=bpy.types.Camera)
    CacheFile : bpy.props.PointerProperty(type=bpy.types.CacheFile)
    Collection : bpy.props.PointerProperty(type=bpy.types.Collection)
    Curve : bpy.props.PointerProperty(type=bpy.types.Curve)
    VectorFont : bpy.props.PointerProperty(type=bpy.types.VectorFont)
    GreasePencil : bpy.props.PointerProperty(type=bpy.types.GreasePencil)
    Image : bpy.props.PointerProperty(type=bpy.types.Image)
    Key : bpy.props.PointerProperty(type=bpy.types.Key)
    Light : bpy.props.PointerProperty(type=bpy.types.Light)
    Library : bpy.props.PointerProperty(type=bpy.types.Library)
    FreestyleLineStyle : bpy.props.PointerProperty(type=bpy.types.FreestyleLineStyle)
    Lattice : bpy.props.PointerProperty(type=bpy.types.Lattice)
    Mask : bpy.props.PointerProperty(type=bpy.types.Mask)
    Material : bpy.props.PointerProperty(type=bpy.types.Material)
    Mesh : bpy.props.PointerProperty(type=bpy.types.Mesh)
    MovieClip : bpy.props.PointerProperty(type=bpy.types.MovieClip)
    NodeTree : bpy.props.PointerProperty(type=bpy.types.NodeTree)
    Object : bpy.props.PointerProperty(type=bpy.types.Object)
    PaintCurve : bpy.props.PointerProperty(type=bpy.types.PaintCurve)
    Palette : bpy.props.PointerProperty(type=bpy.types.Palette)
    ParticleSettings : bpy.props.PointerProperty(type=bpy.types.ParticleSettings)
    LightProbe : bpy.props.PointerProperty(type=bpy.types.LightProbe)
    Scene : bpy.props.PointerProperty(type=bpy.types.Scene)
    Sound : bpy.props.PointerProperty(type=bpy.types.Sound)
    Speaker : bpy.props.PointerProperty(type=bpy.types.Speaker)
    Text : bpy.props.PointerProperty(type=bpy.types.Text)
    Texture : bpy.props.PointerProperty(type=bpy.types.Texture)
    Volume : bpy.props.PointerProperty(type=bpy.types.Volume)
    WindowManager : bpy.props.PointerProperty(type=bpy.types.WindowManager)
    World : bpy.props.PointerProperty(type=bpy.types.World)
    WorkSpace : bpy.props.PointerProperty(type=bpy.types.WorkSpace)
    id_type : bpy.props.EnumProperty(
            items = [
                ('Action', "Action", "", "ACTION", 1),
                ('Armature', "Armature", "", "ARMATURE_DATA", 2),
                ('Brush', "Brush", "", "BRUSH_DATA", 3),
                ('Camera', "Camera", "", "CAMERA_DATA", 4),
                ('CacheFile', "Armature", "", "FILE", 5),
                ('Collection', "Collection", "", "OUTLINER_COLLECTION", 6),
                ('Curve', "Curve", "", "OUTLINER_DATA_CURVE", 7),
                ('VectorFont', "Font", "", "FONT_DATA", 8),
                ('GreasePencil', "Grease Pencil", "", "GREASEPENCIL", 9),
                ('Image', "Image", "", "IMAGE_DATA", 10),
                ('Key', "Key", "", "SHAPEKEY_DATA", 11),
                ('Light', "Light", "", "LIGHT", 12),
                ('Library', "Library", "", "LIBRARY_DATA_DIRECT", 13),
                ('FreestyleLineStyle', "Line Style", "", "LINE_DATA", 14),
                ('Lattice', "Lattice", "", "LATTICE_DATA", 15),
                ('Mask', "Mask", "", "MOD_MASK", 16),
                ('Material', "Material", "", "MATERIAL", 17),
                ('Mesh', "Mesh", "", "MESH_DATA", 18),
                ('MovieClip', "Movie Clip", "", "TRACKER", 19),
                ('NodeTree', "Node Tree", "", "NODETREE", 20),
                ('Object', "Object", "", "OBJECT_DATA", 21),
                ('PaintCurve', "Paint Curve", "", "CURVE_BEZCURVE", 22),
                ('Palette', "Palette", "", "COLOR", 23),
                ('ParticleSettings', "Particle", "", "PARTICLES", 24),
                ('LightProbe', "Light Probe", "", "LIGHTPROBE_CUBEMAP", 25),
                ('Scene', "Scene", "", "SCENE_DATA", 26),
                ('Sound', "Sound", "", "SOUND", 28),
                ('Speaker', "Speaker", "", "SPEAKER", 29),
                ('Text', "Text", "", "TEXT", 30),
                ('Texture', "Texture", "", "TEXTURE", 31),
                ('Volume', "Volume", "", "VOLUME_DATA", 32),
                ('WindowManager', "Window Manager", "", "WINDOW", 33),
                ('World', "World", "", "WORLD", 34),
                ('WorkSpace', "Workspace", "", "WORKSPACE", 35),
                ],
            default = 'Collection',
            name = "ID Type",
            )

    def draw(self, layout, context):
        row = layout.row()
        row.prop(self, "id_type", text="", icon_only=True)
        row.separator(factor=0.1)
        row.prop(self, self.id_type, text="")

    @property
    def value(self):
        return getattr(self, self.id_type)


class ExportDefinition(bpy.types.PropertyGroup):
    name: bpy.props.StringProperty()
    icon: bpy.props.StringProperty()
    items: bpy.props.CollectionProperty(type=ExportObject)

    format_str: bpy.props.StringProperty() # used to store the actual selected file format in string form, so that if a file format is removed or re-ordered, it doesnt get wrong value
    format: bpy.props.EnumProperty(name="Format", items=target_file_format_callback, get=target_file_format_getter, set=target_file_format_setter)

class ConductProperties(bpy.types.PropertyGroup):
    project: bpy.props.StringProperty(name="Project", subtype='FILE_PATH')
    department: bpy.props.StringProperty(name="Department")
    asset: bpy.props.StringProperty(name="Asset")

    exports: bpy.props.CollectionProperty(type = ExportDefinition)
    export_index: bpy.props.IntProperty()
    export_collection_index: bpy.props.IntProperty()

@persistent
def load_handler(dummy):
    print("Resetting conduct data cache")
    global data_cache
    data_cache = None

def register():
    bpy.app.handlers.load_post.append(load_handler)
    bpy.utils.register_class(ExportObject)
    bpy.utils.register_class(ExportDefinition)
    bpy.utils.register_class(ConductProperties)
    bpy.types.Scene.conduct = bpy.props.PointerProperty(type=ConductProperties)
 
 
def unregister():
    bpy.utils.unregister_class(ConductProperties)
    bpy.utils.unregister_class(ExportDefinition)
    bpy.utils.unregister_class(ExportObject)
    bpy.app.handlers.load_post.remove(load_handler)