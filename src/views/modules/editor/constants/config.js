import * as dft from './default';
import * as styleFormConfig from './style-form-config';

// 组件列表的配置项
export const WIDGET_LIST = [
  {
    type: 'text',
    component: 'custom-text',
    label: '文字',
    default: dft.TEXT_DATA,
    styles: dft.TEXT_STYLE,
    styleForm: styleFormConfig.TEXT,
  },
  {
    type: 'video',
    component: 'custom-video',
    label: '视频',
    default: dft.VIDEO_DATA,
    styles: dft.VIDEO_STYLE,
    styleForm: styleFormConfig.VIDEO,
  },
  {
    type: 'table',
    component: 'custom-table',
    label: '表单',
    default: dft.TABLE_DATA,
    styles: dft.TABLE_STYLE,
    styleForm: styleFormConfig.TABLE,
  },
];
