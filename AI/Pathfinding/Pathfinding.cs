using System;
using System.Collections.Generic;

namespace Dungeon_Crawler_World.AI.Pathfinding
{
    public class Pathfinding
    {
        private const int MOVE_COST = 10;
        private const int DIAGONAL_MOVE_COST = 14;

        public static List<Node> FindPath(Node startNode, Node endNode, bool[,] walkableMap)
        {
            List<Node> openList = new List<Node>();
            HashSet<Node> closedList = new HashSet<Node>();

            openList.Add(startNode);

            while (openList.Count > 0)
            {
                Node currentNode = GetLowestFCostNode(openList);
                if (currentNode == endNode)
                {
                    return RetracePath(startNode, endNode);
                }

                openList.Remove(currentNode);
                closedList.Add(currentNode);

                foreach (Node neighbor in GetNeighbors(currentNode, walkableMap))
                {
                    if (!neighbor.Walkable || closedList.Contains(neighbor))
                    {
                        continue;
                    }

                    int newMovementCostToNeighbor = currentNode.GCost + GetDistance(currentNode, neighbor);
                    if (newMovementCostToNeighbor < neighbor.GCost || !openList.Contains(neighbor))
                    {
                        neighbor.GCost = newMovementCostToNeighbor;
                        neighbor.HCost = GetDistance(neighbor, endNode);
                        neighbor.Parent = currentNode;

                        if (!openList.Contains(neighbor))
                        {
                            openList.Add(neighbor);
                        }
                    }
                }
            }

            return new List<Node>(); // Return an empty path if no path is found
        }

        private static Node GetLowestFCostNode(List<Node> nodeList)
        {
            Node lowestFCostNode = nodeList[0];
            foreach (Node node in nodeList)
            {
                if (node.FCost < lowestFCostNode.FCost)
                {
                    lowestFCostNode = node;
                }
            }
            return lowestFCostNode;
        }

        private static List<Node> RetracePath(Node startNode, Node endNode)
        {
            List<Node> path = new List<Node>();
            Node currentNode = endNode;

            while (currentNode != startNode)
            {
                path.Add(currentNode);
                currentNode = currentNode.Parent;
            }

            path.Reverse();
            return path;
        }

        private static List<Node> GetNeighbors(Node node, bool[,] walkableMap)
        {
            List<Node> neighbors = new List<Node>();

            for (int x = -1; x <= 1; x++)
            {
                for (int y = -1; y <= 1; y++)
                {
                    if (x == 0 && y == 0)
                    {
                        continue;
                    }

                    int checkX = node.GridX + x;
                    int checkY = node.GridY + y;

                    if (checkX >= 0 && checkX < walkableMap.GetLength(0) && checkY >= 0 && checkY < walkableMap.GetLength(1))
                    {
                        neighbors.Add(new Node(checkX, checkY, walkableMap[checkX, checkY]));
                    }
                }
            }

            return neighbors;
        }

        private static int GetDistance(Node nodeA, Node nodeB)
        {
            int dstX = Math.Abs(nodeA.GridX - nodeB.GridX);
            int dstY = Math.Abs(nodeA.GridY - nodeB.GridY);

            if (dstX > dstY)
            {
                return DIAGONAL_MOVE_COST * dstY + MOVE_COST * (dstX - dstY);
            }
            return DIAGONAL_MOVE_COST * dstX + MOVE_COST * (dstY - dstX);
        }
    }

    public class Node
    {
        public int GridX { get; }
        public int GridY { get; }
        public bool Walkable { get; }
        public int GCost { get; set; }
        public int HCost { get; set; }
        public Node Parent { get; set; }

        public int FCost => GCost + HCost;

        public Node(int gridX, int gridY, bool walkable)
        {
            GridX = gridX;
            GridY = gridY;
            Walkable = walkable;
        }
    }
}
