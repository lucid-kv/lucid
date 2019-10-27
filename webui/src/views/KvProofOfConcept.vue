<template>
  <div>
    <h1>KV Proof of Concept</h1>

    <GetKey v-model="lucidKey" />
  </div>
</template>

<script>
import GetKey from '@/components/Kv/GetKey'

export default {
  name: 'KvProofOfConcept',
  components: {
    GetKey
  },
  data() {
    return {
      lucidKey: ''
    }
  }
}
</script>
