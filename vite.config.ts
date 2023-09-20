import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import { resolve } from 'path'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver, NaiveUiResolver } from 'unplugin-vue-components/resolvers'
import Unocss from 'unocss/vite'
import { presetAttributify, presetUno, presetIcons } from 'unocss'
import transformerDirectives from '@unocss/transformer-directives'
import transformerVariantGroup from '@unocss/transformer-variant-group'

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: [
      {
        find: '@', replacement: resolve(__dirname, 'src'),
      },
    ],
  },
  plugins: [
    vue({
      script: {
        defineModel: true,
      },
    }),
    vueJsx(),
    Unocss({
      presets: [
        presetAttributify({ }),
        presetUno(),
        presetIcons({
          prefix: '',
          extraProperties: {
            'display': 'inline-block',
            'vertical-align': 'middle',
            // ...
          },
        }),
      ],
      transformers: [
        transformerDirectives(),
        transformerVariantGroup(),

      ],
      rules: [
      ],
    }),
    AutoImport({
      imports: ['vue',
        'pinia',
        '@vueuse/core',
        {
          'naive-ui': [
            'useDialog',
            'useMessage',
            'useNotification',
            'useLoadingBar',
          ],
        }],
      dts: resolve(__dirname, 'src/types/auto-import.d.ts'),
      resolvers: [ElementPlusResolver()],
      eslintrc: {
        enabled: true,
        filepath: resolve(__dirname, 'src/types/.eslintrc-auto-import.json'),
        globalsPropValue: true,
      },
    }),
    Components({
      dts: resolve(__dirname, 'src/types/components.d.ts'),
      resolvers: [ElementPlusResolver(), NaiveUiResolver()],
    }),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri supports es2021
    target: ['es2021', 'chrome100', 'safari13'],
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
    chunkSizeWarningLimit: 4096,
    rollupOptions: {
      output: {
        manualChunks: {
          'highlight.js': ['highlight.js'],
          'pinia': ['pinia', 'pinia-plugin-persistedstate'],
          'vue': ['vue'],
          'xterm': ['xterm', 'xterm-addon-fit'],
          'uuid': ['uuid'],
          'mitt': ['mitt'],
        },
      },
    },
  },
})
