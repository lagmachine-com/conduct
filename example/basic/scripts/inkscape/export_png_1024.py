
class InkscapeDataExport():

    def log(self, object):
        import inkex 
        inkex.utils.debug(object)

    def export(self, effect_context, directory=None, file_name=None, extension=None, items=None, ):
        from tempfile import TemporaryDirectory
        import inkex
        import os

        with TemporaryDirectory(prefix='inkscape-command-') as tmpdir:

            svg_file = inkex.command.write_svg(effect_context.svg, tmpdir, 'input.svg')
            output = os.path.join(directory, file_name + extension)
 
            pages = effect_context.svg.namedview.get_pages()
            index = pages.index(items[0])

            page = items[0]
            
            height = 1024
            ratio = page.width / page.height
            width = round(ratio * height)

            out = inkex.command.inkscape(svg_file,
                                         "--export-filename=%s" % output,
                                         '--export-type=png',
                                         '--export-width=%d' % width,
                                         '--export-height=%d' % height,
                                         '--export-page=%d' % (index + 1))
        