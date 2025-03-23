async (data) => {
    console.log('Ingest running user script!')
    console.log(data)

    var export_result = await conduct.api.doExport(data['department'], data['asset'], data['element'], data['shot'], 'ingest', data['format'])
    console.log(export_result)

    var directory = export_result['directory']
    var name = export_result['recommended_file_name']
    var extension = export_result['file_format']

    var ingest_file = data['new_file']
    var result = directory + "/" + name + extension

    let command = `ffmpeg -i '${ingest_file}' -c:a libvorbis '${result}'`
    os.execute(command)
};
