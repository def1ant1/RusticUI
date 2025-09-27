import path from 'path';
import fs from 'fs';
import { expect } from 'chai';
import sinon from 'sinon';
import { resolvePackageSourceRoot, rustDocFlags } from '../../../scripts/rustDocAutomation.js';
import { getMaterialUiComponentInfo } from './getMaterialUiComponentInfo';

describe('getMaterialUiComponentInfo', () => {
  it('return correct info for material component file', function testMaterialInfo() {
    if (rustDocFlags.shouldSkipArchives) {
      // The Rust-first flow swaps in generated JSON and omits the archived JS files.
      this.skip();
      return;
    }

    const componentInfo = getMaterialUiComponentInfo(
      path.join(resolvePackageSourceRoot('mui-material'), 'src/Button/Button.js'),
    );
    sinon.assert.match(componentInfo, {
      name: 'Button',
      apiPathname: '/material-ui/api/button/',
      muiName: 'MuiButton',
      apiPagesDirectory: sinon.match((value) =>
        value.endsWith(path.join('docs', 'pages', 'material-ui', 'api')),
      ),
    });

    expect(componentInfo.getInheritance('ButtonBase')).to.deep.equal({
      name: 'ButtonBase',
      apiPathname: '/material-ui/api/button-base/',
    });

    let existed = false;
    try {
      fs.readdirSync(path.join(process.cwd(), 'docs/data'));
      existed = true;
      // eslint-disable-next-line no-empty
    } catch (error) {}
    if (existed) {
      const demos = componentInfo.getDemos();
      expect(demos).to.have.lengthOf(2);

      expect(demos[0]).to.deep.include({
        demoPageTitle: 'Button Group',
        demoPathname: '/material-ui/react-button-group/',
      });
      expect(demos[0].filePath).to.include('button-group/button-group.md');

      expect(demos[1]).to.deep.include({
        demoPageTitle: 'Button',
        demoPathname: '/material-ui/react-button/',
      });
      expect(demos[1].filePath).to.include('buttons/buttons.md');
    }
  });
});
