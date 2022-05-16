<template>
  <el-card shadow="never" class="aui-card--fill">
    <el-button @click="handleRun">运行</el-button>
    <el-button @click="anli(1)">获取数据字典</el-button>
    <el-button @click="anli(2)">获取系统配置</el-button>
    <el-button @click="anli(3)">获取用户</el-button>
    <el-container style="height: 100%; border: 2px solid #eee">
      <el-row style="width: 100%">
        <el-col :span="12" style="border-right: 2px solid #eee" >
            <textarea ref="textarea" style="width: 500px; height: 100%;"></textarea>
        </el-col>
        <el-col :span="12">
          <div v-html="result"></div>
        </el-col>
      </el-row>
    </el-container>
  </el-card>
</template>

<script>
// 引入全局实例
import CodeMirror from 'codemirror'
// 核心样式
import 'codemirror/lib/codemirror.css'
// 引入主题后还需要在 options 中指定主题才会生效
import 'codemirror/theme/idea.css'

// 需要引入具体的语法高亮库才会有对应的语法高亮效果
// codemirror 官方其实支持通过 /addon/mode/loadmode.js 和 /mode/meta.js 来实现动态加载对应语法高亮库
// 但 vue 貌似没有无法在实例初始化后再动态加载对应 JS ，所以此处才把对应的 JS 提前引入
import 'codemirror/mode/javascript/javascript.js'
import 'codemirror/mode/css/css.js'

//代码补全提示
import 'codemirror/addon/hint/anyword-hint.js';
import 'codemirror/addon/hint/css-hint.js';
import 'codemirror/addon/hint/html-hint.js';
import 'codemirror/addon/hint/javascript-hint.js';
import 'codemirror/addon/hint/show-hint.css';
import 'codemirror/addon/hint/show-hint.js';

//代码版本差异比较
import 'codemirror/addon/merge/merge.js'
import 'codemirror/addon/merge/merge.css'
import DiffMatchPatch from 'diff-match-patch'

window.diff_match_patch = DiffMatchPatch
window.DIFF_DELETE = -1
window.DIFF_INSERT = 1
window.DIFF_EQUAL = 0

export default {
  name: "Code",

  data() {
    return {
      // 内部真实的内容
      code: 'console.log("hello")',
      // 默认的语法类型
      mode: 'javascript',
      // 编辑器实例
      coder: null,
      result:'',
      foldGutter: true,
      // 默认配置
      options: {
        // 缩进格式
        tabSize: 4,
        // 主题，对应主题库 JS 需要提前引入
        theme: 'idea',
        autocorrect: true,
        matchBrackets: true,
        // 显示行号
        lineNumbers: true,
        line: true,
        extraKeys: {"Ctrl": "autocomplete"},
      },
    }
  },
  mounted () {
    this._initialize()
  },
  methods: {
      // 初始化
      _initialize() {
        // 初始化编辑器实例，传入需要被实例化的文本域对象和默认配置
        this.coder = CodeMirror.fromTextArea(this.$refs.textarea, this.options)
        // 编辑器赋值
        this.coder.setValue(this.code)

        // 支持双向绑定
        this.coder.on('cursorActivity', (coder) => {
          this.code = coder.getValue()
          //coder.showHint();

        })

        // 尝试从父容器获取语法类型
        if (this.language) {
          // 获取具体的语法类型对象
          let modeObj = this._getLanguage(this.language)

          // 判断父容器传入的语法是否被支持
          if (modeObj) {
            this.mode = modeObj.label
          }
        }
      },
      // 获取当前语法类型
      _getLanguage(language) {
        // 在支持的语法类型列表中寻找传入的语法类型
        return this.modes.find((mode) => {
          // 所有的值都忽略大小写，方便比较
          let currentLanguage = language.toLowerCase()
          let currentLabel = mode.label.toLowerCase()
          let currentValue = mode.value.toLowerCase()

          // 由于真实值可能不规范，例如 java 的真实值是 x-java ，所以讲 value 和 label 同时和传入语法进行比较
          return currentLabel === currentLanguage || currentValue === currentLanguage
        })
      },
      /** 按钮操作 */
      handleRun() {

        this.$http.post('/js/run',{code:this.code}).then(({ data: res }) => {
          // eslint-disable-next-line eqeqeq
          if (res.code != 0) {
            return this.$message.error(res.msg)
          }
          this.result = res.data
        }).catch(() => {
        })
      },
    anli(i) {
      this.result = '';
          switch (i){
            case 1:{
              this.code =`
let  data = Cassie.getAllDict();
for(let i=0;i<data.length;i++){
  console.log("name:"+data[i].dict_name);
}
              `;
              this.coder.setValue(this.code)
              break;
            }
            case 2:{
              this.code =`
let  data = Cassie.getConfig();
console.log(data.redis_url);`;
              this.coder.setValue(this.code)
              break;
            }
            case 3:{
              this.code =`
let  data = Cassie.getUserById("1");
console.log(data);`;
              this.coder.setValue(this.code)
              break;
            }
          }
    }
    }
  }
</script>

<style>

</style>