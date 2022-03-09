<template>
  <div>
    <el-button type="primary" @click="completeTask()">{{ $t('process.completeTask') }}</el-button>
    <el-button type="warning" @click="rejectTask()">{{ $t('process.rejectTask') }}</el-button>
    <el-button type="info" @click="delegateTask()">{{ $t('process.entrustTask') }}</el-button>
    <ren-task-delegate v-if="renTaskDelegateVisible" ref="renTaskDelegate"></ren-task-delegate>
    <ren-task-handle v-if="renTaskHandleVisible" ref="renTaskHandle"></ren-task-handle>
  </div>
</template>

<script>
import RenTaskDelegate from './ren-task-delegate'
import RenTaskHandle from './ren-task-handle'
export default {
  name: 'RenProcessRunning',
  data () {
    return {
      // 是否显示退回窗口
      renTaskDelegateVisible: false,
      renTaskHandleVisible: false,
      dataForm: {
        taskId: ''
      }
    }
  },
  components: {
    RenTaskDelegate,
    RenTaskHandle
  },
  created () {
    this.dataForm.taskId = this.$route.params.taskId
  },
  methods: {
    completeTask () {
      this.renTaskHandleVisible = true
      this.$nextTick(() => {
        this.$refs.renTaskHandle.dataForm.taskId = this.dataForm.taskId
        this.$refs.renTaskHandle.handleType = 'complete'
        this.$refs.renTaskHandle.init()
      })
    },
    rejectTask () {
      this.renTaskHandleVisible = true
      this.$nextTick(() => {
        this.$refs.renTaskHandle.dataForm.taskId = this.dataForm.taskId
        this.$refs.renTaskHandle.handleType = 'reject'
        this.$refs.renTaskHandle.init()
      })
    },
    delegateTask () {
      this.renTaskDelegateVisible = true
      this.$nextTick(() => {
        this.$refs.renTaskDelegate.dataForm.taskId = this.dataForm.taskId
        this.$refs.renTaskDelegate.init()
      })
    }
  }
}
</script>
