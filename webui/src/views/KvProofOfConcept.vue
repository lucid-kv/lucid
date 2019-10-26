<template>
  <div>
    <h2>KV Proof of Concept</h2>

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
