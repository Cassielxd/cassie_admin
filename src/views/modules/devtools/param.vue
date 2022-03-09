<template>
  <el-card shadow="never" class="aui-card--fill">
    <div slot="header" class="clearfix">
      <span>代码生成参数配置</span>
      <el-button style="float: right; padding: 3px 0" type="text" @click="updateParam">修改</el-button>
    </div>
    <div class="mod-home">
      <el-row>
        <el-col :span="24">
          <table>
            <tr>
              <td style="width: 200px">默认包名</td>
              <td>{{ param.packageName }}</td>
            </tr>
            <tr>
              <td>默认版本号</td>
              <td>{{ param.version }}</td>
            </tr>
            <tr>
              <td>默认作者</td>
              <td>{{ param.author }}</td>
            </tr>
            <tr>
              <td>作者邮箱</td>
              <td>{{ param.email }}</td>
            </tr>
            <tr>
              <td>后端生成路径</td>
              <td>{{ param.backendPath }}</td>
            </tr>
            <tr>
              <td>前端生成路径</td>
              <td>{{ param.frontendPath }}</td>
            </tr>
          </table>
        </el-col>
      </el-row>
      <param-update v-if="paramUpdateVisible" ref="paramUpdate" @refreshDataList="getGenParam"></param-update>
    </div>
  </el-card>
</template>

<script>
import ParamUpdate from './param-update'
export default {
  data () {
    return {
      paramUpdateVisible: false,
      param: {}
    }
  },
  created () {
    this.getGenParam()
  },
  methods: {
    getGenParam () {
      this.$http.get('/devtools/param/info').then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.param = res.data
      }).catch(() => {})
    },
    // 修改
    updateParam () {
      this.paramUpdateVisible = true
      this.$nextTick(() => {
        this.$refs.paramUpdate.init()
      })
    }
  },
  components: {
    ParamUpdate
  }
}
</script>
