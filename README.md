# Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML, CSS and Typescript.

## Auto-update a cada push

O projeto agora está preparado para publicar uma nova versão Windows a cada push na branch `main` e o app instalado passa a verificar e instalar essa atualização ao abrir.

Fluxo:

1. Cada push em `main` dispara [windows-build.yml](.github/workflows/windows-build.yml).
2. O workflow gera uma versão incremental no formato `0.1.<run_number>`.
3. O GitHub Actions publica um release com os artefatos do Tauri.
4. O app instalado consulta `latest.json` no GitHub Releases e instala a atualização automaticamente.

Antes de isso funcionar de ponta a ponta, você ainda precisa configurar a assinatura do updater:

```bash
npx tauri signer generate -w ~/.tauri/clock-app.key
```

Depois disso:

1. Copie a chave pública gerada e substitua `CONFIGURE_AQUI_A_CHAVE_PUBLICA_DO_UPDATER` em [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json).
2. Salve a chave privada no secret `TAURI_SIGNING_PRIVATE_KEY` do repositório no GitHub.
3. Salve a senha da chave no secret `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.

Sem esses dois secrets, o workflow ainda compila, mas o updater não consegue gerar updates assinados válidos.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
