<template>
  <div class="generator-code">
    <el-dialog :visible.sync="visible" title="生成代码" :close-on-click-modal="false" :close-on-press-escape="false">
      <el-form :model="dataForm" :rules="dataRule" ref="dataForm" label-width="110px" size="small">
        <el-row>
          <el-col :span="12">
            <el-form-item label="表名" prop="tableName">
              <el-input v-model="dataForm.tableName" disabled placeholder="表名"></el-input>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="功能名" prop="tableComment">
              <el-input v-model="dataForm.tableComment" placeholder="功能名"></el-input>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="类名" prop="className">
              <el-input v-model="dataForm.className" placeholder="类名"></el-input>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item prop="baseclassId" label="基类">
              <el-select v-model="dataForm.baseclassId" placeholder="基类" class="w-percent-100" clearable>
                <el-option v-for="item in baseClassList" :key="item.id" :label="item.code" :value="item.id"></el-option>
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="模块名" prop="moduleName">
              <el-input v-model="dataForm.moduleName" placeholder="模块名"></el-input>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="子模块名" prop="subModuleName">
              <el-input v-model="dataForm.subModuleName" placeholder="子模块名"></el-input>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="项目包名" prop="packageName">
              <el-input v-model="dataForm.packageName" placeholder="项目包名"></el-input>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="版本号" prop="version">
              <el-input v-model="dataForm.version" placeholder="版本号"></el-input>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="默认作者" prop="author">
              <el-input v-model="dataForm.author" placeholder="默认作者"></el-input>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="作者邮箱" prop="email">
              <el-input v-model="dataForm.email" placeholder="作者邮箱"></el-input>
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="后端访问路径" prop="backendUrl">
          <el-input v-model="dataForm.backendUrl" placeholder="后端访问路径"></el-input>
        </el-form-item>
        <el-form-item label="后端生成路径" prop="backendPath">
          <el-input v-model="dataForm.backendPath" placeholder="后端生成路径"></el-input>
        </el-form-item>
        <el-form-item label="前端生成路径" prop="frontendPath">
          <el-input v-model="dataForm.frontendPath" placeholder="前端生成路径"></el-input>
        </el-form-item>
      </el-form>
      <template slot="footer">
        <el-button @click="visible = false" size="small">取消</el-button>
        <el-button type="primary" @click="dataFormSubmitHandle" size="small">保存</el-button>
        <el-button type="danger" @click="generatorCodeHandle" size="small">生成代码</el-button>
      </template>
    </el-dialog>
  </div>
</template>
<script>
import debounce from 'lodash/debounce'
export default {
  data () {
    return {
      visible: false,
      baseClassList: [],
      dataForm: {
        id: '',
        baseclassId: '',
        backendPath: '',
        frontendPath: '',
        backendUrl: '',
        packageName: ''
      }
    }
  },
  computed: {
    dataRule () {
      return {
        tableName: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        tableComment: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        className: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        packageName: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        author: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        backendUrl: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        backendPath: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        frontendPath: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ]
      }
    }
  },
  methods: {
    init (id) {
      this.visible = true
      this.$nextTick(() => {
        this.$refs['dataForm'].resetFields()
        this.getBaseClassList()
        this.getTableInfo(id)
      })
    },
    getBaseClassList () {
      this.$http.get('/devtools/baseclass/list').then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.baseClassList = res.data
      }).catch(() => {})
    },
    getTableInfo (id) {
      this.$http.get('/devtools/table/' + id).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm = {
          ...this.dataForm,
          ...res.data
        }
      }).catch(() => {})
    },
    selectDeveloper (id) {
      this.$http.get('/devtools/developer/' + id).then(({ data: res }) => {
        // self.dataForm.outPath = res.data.data.outPath;
      }).catch(() => {})
    },
    // 表单提交
    dataFormSubmitHandle: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        this.$http['put']('/devtools/table', this.dataForm).then(({ data: res }) => {
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
    }, 1000, { 'leading': true, 'trailing': false }),
    // 表单提交
    generatorCodeHandle: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        this.$http['post']('/devtools/generator', this.dataForm).then(({ data: res }) => {
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
.generator-code .el-dialog__body{
  padding: 15px 30px 0 20px;
}
</style>