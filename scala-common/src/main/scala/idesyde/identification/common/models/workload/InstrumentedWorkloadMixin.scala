package idesyde.identification.common.models.workload

trait InstrumentedWorkloadMixin {

  def processComputationalNeeds: Array[Map[String, Map[String, Long]]]
  def processSizes: Array[Long]

  def messagesMaxSizes: Array[Long]

}
