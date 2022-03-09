<template>
  <div class="template-code">
  <el-dialog :visible.sync="visible" :title='!dataForm.id ? "新增" : "编辑"' :close-on-click-modal="false" :close-on-press-escape="false" :fullscreen="true">
    <el-form :model="dataForm" :rules="dataRule" ref="dataForm" label-width="80px" size="mini">
      <el-row :gutter="24">
        <el-col :span="12">
          <el-form-item label="模板名" prop="name">
            <el-input v-model="dataForm.name" placeholder="模板名"></el-input>
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="文件名" prop="fileName">
            <el-input v-model="dataForm.fileName" placeholder="文件名"></el-input>
          </el-form-item>
        </el-col>
      </el-row>
      <el-row :gutter="24">
        <el-col :span="12">
          <el-form-item label="生成路径" prop="path">
            <el-input v-model="dataForm.path" placeholder="生成路径"></el-input>
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="状态" prop="status">
            <el-radio-group v-model="dataForm.status">
              <el-radio :label="0">启用</el-radio>
              <el-radio :label="1">禁用</el-radio>
            </el-radio-group>
          </el-form-item>
        </el-col>
      </el-row>
      <el-form-item label="内容" prop="content">
        <MonacoEditor class="editor" v-model="dataForm.content" theme="vs" language="java" style="height: 480px;border:1px solid #ccc" />
      </el-form-item>
    </el-form>
    <template slot="footer">
      <el-button @click="visible = false">取消</el-button>
      <el-button type="primary" @click="dataFormSubmitHandle()">确定</el-button>
    </template>
  </el-dialog>
  </div>
</template>
<script>
import debounce from 'lodash/debounce'
import MonacoEditor from 'vue-monaco'
export default {
  data () {
    return {
      visible: false,
      dataForm: {
        id: '',
        name: '',
        content: '',
        fileName: '',
        path: '',
        status: 0
      }
    }
  },
  computed: {
    dataRule () {
      return {
        name: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        content: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        path: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        fileName: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ]
      }
    }
  },
  components: {
    MonacoEditor
  },
  methods: {
    init () {
      this.visible = true
      this.$nextTick(() => {
        this.$refs['dataForm'].resetFields()
        if (this.dataForm.id) {
          this.getInfo()
        }
      })
    },
    // 获取信息
    getInfo () {
      this.$http.get(`/devtools/template/${this.dataForm.id}`).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm = {
          ...this.dataForm,
          ...res.data
        }
      }).catch(() => {})
    },
    // 表单提交
    dataFormSubmitHandle: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        this.$http[!this.dataForm.id ? 'post' : 'put']('/devtools/template', this.dataForm).then(({ data: res }) => {
          if (res.code !== 0) {
            return this.$message.error(res.msg)
          }
          this.$message({
            message: this.$t('prompt.success'),
            type: 'success',
            duration: 500,
            onClose: () => {
              this.visible = false
              this.$emit('refreshDataList')
            }
          })
        }).catch(() => {})
      })
    }, 1000, { 'leading': true, 'trailing': false })
  }
}
</script>
<style>
.template-code .el-dialog__body{
  padding: 10px 30px 0 20px;
}
.template-code .el-dialog__footer {
  padding: 0px 20px 10px;
}
</style>