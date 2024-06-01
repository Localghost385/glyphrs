import * as vscode from 'vscode'

export async function activate(context: vscode.ExtensionContext) {
  const disposable = vscode.commands.registerCommand('glyphrs.insert', () => {
    const editor = vscode.window.activeTextEditor
    if (!editor) {
      return
    }

    import('../core').then(async (module) => {
      try {
        let content: string | undefined
        let font: string | undefined
        let prefix: string | undefined

        await vscode.window
          .showInputBox({
            prompt: 'content',
            placeHolder: 'content',
          })
          .then(async (contentValue) => {
            content = contentValue
            const fontOptions = module.get_fonts()
            await vscode.window
              .showQuickPick([...fontOptions])
              .then(async (fontValue) => {
                font = fontValue
                await vscode.window
                  .showInputBox({
                    prompt: 'prefix',
                    placeHolder: 'prefix',
                  })
                  .then((prefixValue) => {
                    prefix = prefixValue
                  })
              })
          })

        if (!content) {
          content = ''
        }
        if (!font) {
          font = 'pipes'
        }
        if (!prefix) {
          prefix = ''
        }
        const result = module.convert_input(content, font, prefix)
        let output = ''
        result.forEach((line) => {
          output += line + '\n'
        })

        editor.edit((edit) => {
          //insert the result at the current cursor position
          edit.insert(editor.selection.active, output)
        })
      } catch (e) {
        vscode.window.showErrorMessage(['cannot format the file.', e].join(' '))
      }
    })
  })
  context.subscriptions.push(disposable)
}

export function deactivate() {
  return
}
