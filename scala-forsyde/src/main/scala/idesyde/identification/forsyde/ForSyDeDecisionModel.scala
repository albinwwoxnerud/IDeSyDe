package idesyde.identification.forsyde

import forsyde.io.java.core.ForSyDeSystemGraph
import forsyde.io.java.core.Vertex
import idesyde.identification.DecisionModel
import forsyde.io.java.core.EdgeInfo

trait ForSyDeDecisionModel extends DecisionModel {

  type ElementT         = Vertex
  type ElementRelationT = EdgeInfo

}
