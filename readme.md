# GESTOR OS (v3.0.0)

Sistema desktop de alta performance desenvolvido para emissão e gerenciamento completo de ordens de serviço.

## 🚀 Tecnologias (Migrado na v3.0.0)

- **Tauri v2** (Nativo, leve e ultraveloz)
- **Rust** (Backend nativo e seguro)
- **SQLite3** (rusqlite nativo com modo WAL)
- **Frontend Moderno** (HTML5, CSS3 Tecnológico, JavaScript SPA)
- **Auto-Updater Nativo** (Verificação automática e manual de atualizações com assinaturas criptográficas)
- **Consumo de Memória:** ~30MB a 35MB de RAM (queda drástica de ~500MB no antigo Electron)

## 📦 Como rodar o projeto em ambiente de desenvolvimento

### Pré-requisitos
- Node.js (v18+)
- Rust & Cargo (1.90+)

### Instalar dependências
```bash
npm install
```

### Configurar variáveis de ambiente
Copie o arquivo `.env.example` para `.env`:
```bash
cp .env.example .env
```

### Iniciar em modo desenvolvimento
```bash
npm run dev
```

### Gerar binário / instalador de produção (com auto-updater e manifesto)
```bash
npm run build
```

## 🖥️ Instalação no PC do Cliente

1. Acesse [Releases](https://github.com/fesalgz/gestor_os/releases) no GitHub.
2. Baixe o instalador da **versão 3.0.0** (`Gestor OS_3.0.0_x64-setup.exe`).
3. Siga os passos do assistente de instalação.
4. O aplicativo abrirá instantaneamente consumindo menos de 40MB de memória RAM, preservando integralmente o banco de dados das versões anteriores.
