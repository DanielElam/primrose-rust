// start the process 'C:\Primrose\PrimroseEngine\Players\Player.Windows\bin\Debug\Player.exe'

function startProcess() {
    var spawn = require('child_process').spawn;
    var process = spawn('C:\\Primrose\\PrimroseEngine\\Players\\Player.Windows\\bin\\Debug\\Player.exe', [], { detached: true });
    process.unref();
}

startProcess();