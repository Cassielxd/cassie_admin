<template>
  <el-card shadow="never" class="aui-card--fill">
    <el-form :model="dataForm" :rules="dataRule" ref="dataForm" @keyup.enter.native="dataFormSubmitHandle()" :label-width="$i18n.locale === 'en-US' ? '120px' : '80px'">
      <el-form-item :label="$t('correction.post')" prop="applyPost">
        <el-input v-model="dataForm.applyPost" :disabled="fieldDisabled" :placeholder="$t('correction.post')"></el-input>
      </el-form-item>
      <el-row :gutter="40">
        <el-col :span="12">
          <el-form-item :label="$t('correction.entryDate')" prop="entryDate">
            <el-date-picker v-model="dataForm.entryDate" :disabled="fieldDisabled" value-format="yyyy-MM-dd" :placeholder="$t('correction.entryDate')" style="width: 100%"></el-date-picker>
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item :label="$t('correction.correctionDate')" prop="correctionDate">
            <el-date-picker v-model="dataForm.correctionDate" :disabled="fieldDisabled" value-format="yyyy-MM-dd" :placeholder="$t('correction.correctionDate')" style="width: 100%"></el-date-picker>
          </el-form-item>
        </el-col>
      </el-row>
      <el-form-item :label="$t('correction.workContent')" prop="workContent">
        <el-input type="textarea" v-model="dataForm.workContent" :disabled="fieldDisabled" :placeholder="$t('correction.workContent')"></el-input>
      </el-form-item>
      <el-form-item :label="$t('correction.achievement')" prop="achievement">
        <el-input type="textarea" v-model="dataForm.achievement" :disabled="fieldDisabled" :placeholder="$t('correction.achievement')"></el-input>
      </el-form-item>
    </el-form>
    <template>
      <el-row :gutter="640" v-if="!fieldDisabled">
        <el-col :span="4" :offset="1">
          <el-button type="primary" @click="dataFormSubmitHandle()">{{ $t('process.createInstance') }}</el-button>
        </el-col>
      </el-row>
      <ren-process-running v-if="runningHandleVisible" ref="renProcessRunning"></ren-process-running>
      <ren-process-detail v-if="processDetailVisible" ref="renProcessDetail"></ren-process-detail>
    </template>
  </el-card>
</template>

<script>
import mixinViewModule from '@/mixins/view-module'
import debounce from "lodash/debounce";
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        createdIsNeed: false
      },
      // 表单属性是否可编辑
      fieldDisabled: false,
      // 是否任务处理
      runningHandleVisible: false,
      // 是否显示流程处理详情
      processDetailVisible: false,
      dataForm: {
        id: '',
        applyPost: '',
        entryDate: '',
        correctionDate: '',
        workContent: '',
        achievement: '',
        //必传
        processDefinitionId: ''
      }
    }
  },
  created () {
    this.dataForm.id = this.$route.params.businessKey
    this.dataForm.processDefinitionId = this.$route.params.processDefinitionId
    const showType = this.$route.params.showType

    this.$nextTick(() => {
      this.$refs['dataForm'].resetFields()
      //表单已存在，不允许编辑
      if (this.dataForm.id) {
        this.fieldDisabled = true
        this.getInfo()
      }

      //查看流程
      if (showType === 'detail') {
        this.processDetailVisible = true
      }else if (showType === 'taskHandle') { //处理流程
        this.runningHandleVisible = true
      }

    })
  },
  computed: {
    dataRule () {
      return {
        applyPost: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        entryDate: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        correctionDate: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        workContent: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        achievement: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        createTime: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ]
      }
    }
  },
  methods: {
    // 启动流程
    dataFormSubmitHandle: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        this.$http[!this.dataForm.id ? 'post' : 'put']('/flow/demo/correction/start', this.dataForm).then(({ data: res }) => {
          if (res.code !== 0) {
            return this.$message.error(res.msg)
          }
          this.$message({
            message: this.$t('prompt.success'),
            type: 'success',
            duration: 500,
            onClose: () => {
              this.closeCurrentTab()
            }
          })
        }).catch(() => {})
      })
    }, 1000, { 'leading': true, 'trailing': false }),
    // 获取表单信息
    getInfo () {
      this.$http.get(`/flow/demo/correction/${this.dataForm.id}`).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm = {
          ...this.dataForm,
          ...res.data
        }
      }).catch(() => {})
    }
  }
}
</script>
