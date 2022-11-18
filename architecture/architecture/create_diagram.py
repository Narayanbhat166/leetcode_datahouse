from diagrams import Diagram, Cluster, Edge
from diagrams.aws.compute import EC2
from diagrams.aws.database import RDS
from diagrams.aws.storage import ElasticFileSystemEFS
from diagrams.onprem.inmemory import Redis
from diagrams.onprem.compute import Server

with Diagram("Leetcode Datahouse", show=False):
    producers = []
    with Cluster("Producers"):
        producers.append(EC2("scrapper"))
        producers.append(EC2("scrapper"))
        producers.append(EC2("scrapper"))
    redis = Redis("Redis")
    db = RDS("Relational DB")
    consumer = EC2("consumer")
    lc_server = Server("Leetcode server")
    file_system = ElasticFileSystemEFS("File system")

    redis - Edge(label="submissions") >> consumer
    consumer - Edge(label="persist code hash") >> db
    for producer in producers:
        producer - Edge(label="submission") >> redis
        lc_server - Edge(label="submission") >> producer
    consumer - Edge(label="submitted code") >> file_system
        
