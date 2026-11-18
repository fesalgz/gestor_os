const fs = require('fs');
const path = require('path');

function generateManifest() {
    const tauriConfPath = path.join(__dirname, '..', 'src-tauri', 'tauri.conf.json');
    if (!fs.existsSync(tauriConfPath)) {
        console.error('[Manifest] Erro: tauri.conf.json não encontrado.');
        process.exit(1);
    }

    const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf-8'));
    const version = tauriConf.version || '1.0.0';
    const nsisDir = path.join(__dirname, '..', 'src-tauri', 'target', 'release', 'bundle', 'nsis');
    const bundleDir = path.join(__dirname, '..', 'src-tauri', 'target', 'release', 'bundle');

    if (!fs.existsSync(nsisDir)) {
        console.warn('[Manifest] Pasta NSIS não encontrada em:', nsisDir);
        return;
    }

    // Procura o arquivo .exe e o arquivo .sig mais recentes
    const files = fs.readdirSync(nsisDir);
    const exeFile = files.find(f => f.endsWith('-setup.exe') && !f.includes('Ordens de'));
    const sigFile = files.find(f => f.endsWith('-setup.exe.sig') && !f.includes('Ordens de'));

    if (!exeFile || !sigFile) {
        console.warn('[Manifest] Arquivo .exe ou .sig não encontrado em:', nsisDir);
        return;
    }

    const githubRepo = process.env.GITHUB_REPO || 'fesalgz/gestor_os';
    const sigContent = fs.readFileSync(path.join(nsisDir, sigFile), 'utf-8').trim();
    const downloadUrl = `https://github.com/${githubRepo}/releases/download/v${version}/${encodeURIComponent(exeFile)}`;

    const manifest = {
        version: version,
        notes: `Atualização automática do Gestor OS v${version}`,
        pub_date: new Date().toISOString(),
        platforms: {
            "windows-x86_64": {
                signature: sigContent,
                url: downloadUrl
            }
        }
    };

    const manifestJson = JSON.stringify(manifest, null, 2);

    // Salva em src-tauri/target/release/bundle/latest.json
    const outBundlePath = path.join(bundleDir, 'latest.json');
    fs.writeFileSync(outBundlePath, manifestJson, 'utf-8');

    // Salva também dentro de bundle/nsis/latest.json por conveniência
    const outNsisPath = path.join(nsisDir, 'latest.json');
    fs.writeFileSync(outNsisPath, manifestJson, 'utf-8');

    console.log('\n======================================================');
    console.log('[Manifest] latest.json gerado com sucesso!');
    console.log(`Versão: v${version}`);
    console.log(`Instalador: ${exeFile}`);
    console.log(`URL configurada: ${downloadUrl}`);
    console.log(`Arquivos salvos em:`);
    console.log(`  -> ${outBundlePath}`);
    console.log(`  -> ${outNsisPath}`);
    console.log('======================================================\n');
}

generateManifest();

