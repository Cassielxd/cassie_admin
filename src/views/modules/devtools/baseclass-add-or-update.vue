<template>
  <el-dialog :visible.sync="visible" :title='!dataForm.id ? "新增" : "编辑"' :close-on-click-modal="false" :close-on-press-escape="false">
    <el-form :model="dataForm" :rules="dataRule" ref="dataForm" @keyup.enter.native="dataFormSubmitHandle()" label-width="120px">
      <el-form-item label="基类编码" prop="code">
        <el-input v-model="dataForm.code" placeholder="基类编码"></el-input>
      </el-form-item>
      <el-form-item label="基类包名" prop="packageName">
        <el-input v-model="dataForm.packageName" placeholder="基类包名"></el-input>
      </el-form-item>
      <el-form-item label="基类字段" prop="fields">
        <el-input v-model="dataForm.fields" placeholder="基类字段，多个字段，用英文逗号分隔"></el-input>
      </el-form-item>
      <el-form-item label="备注" prop="remark">
        <el-input v-model="dataForm.remark" placeholder="备注"></el-input>
      </el-form-item>
    </el-form>
    <template slot="footer">
      <el-button @click="visible = false">取消</el-button>
      <el-button type="primary" @click="dataFormSubmitHandle()">确定</el-button>
    </template>
  </el-dialog>
</template>
<script>
import debounce from 'lodash/debounce'
export default {
  data () {
    return {
      visible: false,
      dataForm: {
        id: '',
        packageName: '',
        code: '',
        fields: '',
        remark: ''
      }
    }
  },
  computed: {
    dataRule () {
      return {
        packageName: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        code: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        fields: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ]
      }
    }
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
      this.$http.get(`/devtools/baseclass/${this.dataForm.id}`).then(({ data: res }) => {
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
        this.$http[!this.dataForm.id ? 'post' : 'put']('/devtools/baseclass', this.dataForm).then(({ data: res }) => {
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
