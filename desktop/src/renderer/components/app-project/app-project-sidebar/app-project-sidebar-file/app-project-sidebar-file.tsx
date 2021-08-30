import { Component, h, Host, Prop, State } from '@stencil/core'
import { taskEither as TE } from 'fp-ts'
import { pipe } from 'fp-ts/function'
import { File } from 'stencila'
import { state } from '../../../../store'
import { openDocumentInActivePane } from '../../../../store/documentPane/documentPaneActions'
import { selectProjectFile } from '../../../../store/project/projectSelectors'
import { errorToast } from '../../../../utils/errors'
import { getFileIcon } from './iconMap'

@Component({
  tag: 'app-project-sidebar-file',
  styleUrl: 'app-project-sidebar-file.css',
  scoped: true,
})
export class AppProjectSidebarFile {
  @Prop()
  filePath: string

  @Prop()
  isMain: boolean = false

  @State()
  isCollapsed = true

  private file: File | undefined

  private clickHandler = (e: MouseEvent) => {
    e.preventDefault()

    if (this.file?.children) {
      this.isCollapsed = !this.isCollapsed
    } else {
      pipe(this.filePath, openDocumentInActivePane, TE.mapLeft(errorToast))()
    }
  }

  componentWillLoad() {
    this.file = selectProjectFile(state)(this.filePath)
  }

  render() {
    const file = selectProjectFile(state)(this.filePath)

    if (!file) return

    const isDir = file?.children !== undefined

    return (
      <Host>
        <li>
          <a
            href="#"
            class={{
              isDir,
              isFile: !isDir,
              isMain: this.isMain,
            }}
            onClick={this.clickHandler}
          >
            <stencila-icon
              icon={getFileIcon(file, this.isCollapsed, this.isMain)}
              iconStyle="fill"
            ></stencila-icon>
            <span>{file?.name}</span>
          </a>
          {!this.isCollapsed && file?.children && (
            <ul>
              {file.children.map((filePath) => (
                <app-project-sidebar-file
                  filePath={filePath}
                ></app-project-sidebar-file>
              ))}
            </ul>
          )}
        </li>
      </Host>
    )
  }
}
