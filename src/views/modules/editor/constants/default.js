


// 文字默认数据
export const TEXT_DATA = {
  w: 200,
  h: 50,
  value: 'hello world!',
};

// 文字默认样式
export const TEXT_STYLE = {
  color: '#000000',
  fontSize: 24,
};

// 视频默认数据
export const VIDEO_DATA = {
  w: 400,
  h: 300,
  value: 'https://cdn.theguardian.tv/webM/2015/07/20/150716YesMen_synd_768k_vp8.webm',
};

// 视频默认样式
export const VIDEO_STYLE = {
  // 是否显示控制条
  ctrlBarVisible: true,
};

export const TABLE_DATA = {
  w: 600,
  h: 300,
  value: {
    tableData: [{
      date: '2016-05-02',
      name: '王小虎',
      address: '上海市普陀区金沙江路 1518 弄'
    }, {
      date: '2016-05-04',
      name: '王小虎',
      address: '上海市普陀区金沙江路 1517 弄'
    }, {
      date: '2016-05-01',
      name: '王小虎',
      address: '上海市普陀区金沙江路 1519 弄'
    }, {
      date: '2016-05-03',
      name: '王小虎',
      address: '上海市普陀区金沙江路 1516 弄'
    }]
  },
};

// 视频默认样式
export const TABLE_STYLE = {
  // 是否显示斑马线
  stripe: true,
  tableColumns:[
    {
      columnCode:'name',
      columnName:'姓名',
      width:'180'
    },
    {
      columnCode:'date',
      columnName:'日期',
      width:'180'
    },

    {
      columnCode:'address',
      columnName:'地址',
      width:'180'
    }
  ]
};
