package idesyde.devicetree.identification

import idesyde.blueprints.IdentificationModule
import idesyde.core.DecisionModel
import idesyde.core.DesignModel
import idesyde.utils.Logger
import idesyde.core.headers.DesignModelHeader
import idesyde.core.headers.DecisionModelHeader

object DeviceTreeIdentificationModule extends IdentificationModule with PlatformRules {

  def decisionModelDecoders: Set[DecisionModelHeader => Option[DecisionModel]] = Set()

  def designModelDecoders: Set[DesignModelHeader => Set[DesignModel]] = Set()

  def uniqueIdentifier: String = "DeviceTreeIdentificationModule"

  def identificationRules
      : Set[(Set[DesignModel], Set[DecisionModel]) => Set[? <: DecisionModel]] = Set(
    identSharedMemoryMultiCore,
    identPartitionedCoresWithRuntimes
  )

  def integrationRules: Set[(DesignModel, DecisionModel) => Option[? <: DesignModel]] =
    Set()

}
