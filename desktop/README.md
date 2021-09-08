# 🖥️ Stencila Desktop

**Use Stencila on your own computer**

![](screenshot.png)

## 📦 Install

The desktop client is is early stages of development (all contributions welcome!). We don't necessarily recommend installing it yet, but if you are an early adopter, we'd also appreciate any feedback 💖. You can download standalone binaries for MacOS, Windows or Linux from the [latest release](https://github.com/stencila/stencila/releases/latest):

- Windows: `Stencila-<version>.Setup.exe`
- MacOS: `Stencila-darwin-x64-<version>.zip`
- Linux: `stencila-desktop_<version>_amd64.deb` or `stencila-desktop-<version>.x86_64.rpm`

## 🛠️ Develop

The desktop client is built using [ElectronJS](https://www.electronjs.org), and uses [StencilJS](https://stenciljs.com) for the UI. We use [Electron Forge](https://stenciljs.com) for bootstrapping the project.

The desktop client uses the [Stencila Node bindings](../node) under the hood, so make sure you have
followed the [development installations steps there](../node#%EF%B8%8F-develop). In particular, you'll have to run `npm run build` in the `../node` folder so that it is available as a built package here.

Once ready, run the following commands.

```sh
npm install
npm run ui:start

# In another terminal window
npm run start
```

### Environment variables

We use `.env` files for injection environment variables into the build.
There is an `.env.example` file which you can duplicate and rename to `.env`.
In the example file you will find all the available environment variables which
you can override.

### Building binaries

To build binaries use the following command. Note that the binaries will be tied
to the operating system used to build them.

```sh
npm run make
```
