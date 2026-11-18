mod commands;
mod db;
mod models;

use std::sync::Mutex;
use tauri::Manager;

#[tauri::command]
fn get_database_info(state: tauri::State<db::DatabaseState>) -> Result<db::DatabaseInfo, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    let clientes_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM clientes", [], |r| r.get(0))
        .unwrap_or(0);
    let ordens_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM ordens", [], |r| r.get(0))
        .unwrap_or(0);
    let usuarios_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM usuarios", [], |r| r.get(0))
        .unwrap_or(0);

    let (empresa_nome, nome_sistema): (Option<String>, Option<String>) = conn
        .query_row(
            "SELECT empresa_nome, nome_sistema FROM configuracoes WHERE id = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap_or((None, None));

    Ok(db::DatabaseInfo {
        db_path: state.path.to_string_lossy().to_string(),
        clientes_count,
        ordens_count,
        usuarios_count,
        empresa_nome,
        nome_sistema,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Diretório padrão de dados do Tauri para o Gestor OS
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            let target_db_dir = app_data_dir.join("database");
            let target_db = target_db_dir.join("sistema.db");

            // Migração automática: se o banco ainda não existir na nova pasta
            if !target_db.exists() && !app_data_dir.join("sistema.db").exists() {
                let _ = std::fs::create_dir_all(&target_db_dir);

                let mut legacy_candidates = Vec::new();
                if let Some(roaming) = dirs::data_dir() {
                    // 1. %APPDATA%/ods_cobrinha/
                    let ods_dir = roaming.join("ods_cobrinha");
                    legacy_candidates.push(ods_dir.join("database").join("sistema.db"));
                    legacy_candidates.push(ods_dir.join("sistema.db"));
                    legacy_candidates.push(ods_dir.join("sistema.sqlite"));

                    if ods_dir.exists() {
                        if let Ok(entries) = std::fs::read_dir(&ods_dir) {
                            for entry in entries.flatten() {
                                let p = entry.path();
                                if let Some(ext) = p.extension() {
                                    if ext == "db" || ext == "sqlite" || ext == "sqlite3" {
                                        legacy_candidates.push(p);
                                    }
                                }
                            }
                        }
                    }

                    // 2. %APPDATA%/ordens-de-servico-cobrinha/
                    let electron_dir = roaming.join("ordens-de-servico-cobrinha");
                    legacy_candidates.push(electron_dir.join("database").join("sistema.db"));
                    legacy_candidates.push(electron_dir.join("sistema.db"));
                    legacy_candidates.push(electron_dir.join("sistema.sqlite"));
                }

                // 3. database/sistema.db local (ambiente dev)
                legacy_candidates.push(std::path::PathBuf::from("database").join("sistema.db"));

                for candidate in legacy_candidates {
                    if candidate.exists() && candidate != target_db {
                        log::info!("Migrando banco legado de {:?} para {:?}", candidate, target_db);
                        if std::fs::copy(&candidate, &target_db).is_ok() {
                            log::info!("Migração concluída com sucesso para {:?}", target_db);
                            break;
                        }
                    }
                }
            }

            let effective_db = if app_data_dir.join("sistema.db").exists() && !target_db.exists() {
                app_data_dir.join("sistema.db")
            } else if target_db.exists() {
                target_db
            } else {
                db::resolve_db_path()
            };

            let conn = db::init_db(&effective_db).expect("Falha ao inicializar o banco de dados SQLite");
            let db_state = db::DatabaseState {
                conn: Mutex::new(conn),
                path: effective_db,
            };
            app.manage(db_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_database_info,
            commands::get_dashboard_data,
            commands::listar_clientes,
            commands::get_cliente,
            commands::salvar_cliente,
            commands::atualizar_cliente,
            commands::excluir_cliente,
            commands::listar_ordens,
            commands::get_ordem,
            commands::salvar_ordem,
            commands::atualizar_ordem,
            commands::finalizar_ordem,
            commands::excluir_ordem,
            commands::get_dados_impressao,
            commands::listar_funcionarios,
            commands::listar_nomes_os,
            commands::salvar_funcionario,
            commands::atualizar_funcionario,
            commands::excluir_funcionario,
            commands::get_configuracoes,
            commands::get_logo_base64,
            commands::salvar_logo,
            commands::remover_logo,
            commands::toggle_tema,
            commands::salvar_identidade,
            commands::salvar_nome_sistema,
            commands::verificar_senha_painel,
            commands::alterar_senha_painel,
            commands::get_tamanho_banco,
            commands::limpar_ordens_servico,
            commands::limpar_todos_clientes,
            commands::exportar_backup,
            commands::restaurar_backup,
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
