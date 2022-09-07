import * as dft from './default'
import * as styleFormConfig from './style-form-config'
import { IMAGE_DATA } from './default'
import { IMAGE } from './style-form-config'

// 组件列表的配置项
export const WIDGET_LIST = [
  {
    type: 'text',
    component: 'custom-text',
    label: '文字',
    default: dft.TEXT_DATA,
    styles: dft.TEXT_STYLE,
    styleForm: styleFormConfig.TEXT,
    handles: ['tl', 'tm', 'tr', 'mr', 'br', 'bm', 'bl', 'ml'],
  },
  {
    type: 'video',
    component: 'custom-video',
    label: '视频',
    default: dft.VIDEO_DATA,
    styles: dft.VIDEO_STYLE,
    styleForm: styleFormConfig.VIDEO,
    handles: ['tl', 'tm', 'tr', 'mr', 'br', 'bm', 'bl', 'ml'],
  },
  {
    type: 'table',
    component: 'custom-table',
    label: '列表',
    default: dft.TABLE_DATA,
    styles: dft.TABLE_STYLE,
    styleForm: styleFormConfig.TABLE,
    handles: ['tl', 'tm', 'tr', 'mr', 'br', 'bm', 'bl', 'ml'],
  },
  {
    type: 'image',
    component: 'custom-image',
    label: '图片',
    default: dft.IMAGE_DATA,
    styles: dft.IMAGE_STYLE,
    styleForm: styleFormConfig.IMAGE,
    handles: ['tl', 'tm', 'tr', 'mr', 'br', 'bm', 'bl', 'ml'],
  }, {
    type: 'hline',
    component: 'custom-hline',
    label: '横线',
    default: {
      w: 200,
      h: 4,
    },
    handles: ['mr','ml'],
    styles: {},
    styleForm: [],
  },{
    type: 'vline',
    component: 'custom-vline',
    label: '竖线',
    default: {
      w: 4,
      h: 200,
    },
    handles: ['tm','bm'],
    styles: {},
    styleForm: [],
  },
]
