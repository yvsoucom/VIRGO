# VIRGO  
**A Next-Generation Hierarchical Distributed Architecture**
VIRGO has now been **redesigned and re-implemented in Rust** to take advantage of Rust’s performance, safety, and concurrency features.
---

## Overview

**VIRGO** is a virtual hierarchical distributed system designed for **scalable, fault-tolerant, and multi-role communication** over heterogeneous networks.

Unlike unstructured or strictly structured P2P networks, VIRGO represents a third type of P2P architecture that preserves the semantic meaning of nodes and their parent groups through a virtual hierarchical tree.

VIRGO allows each physical node to host multiple logical nodes across different layers of this virtual tree, enabling a single device to dynamically assume roles ranging from leaf participant to regional coordinator, and even global root.
VIRGO supports two complementary deployment modes:

- **VIRGO (Overlay Mode):**  
  A virtual hierarchical overlay built on top of existing IP networks.
- **sVIRGO (Substrate Mode):**  
  A non-overlay architecture where hierarchical routing and coordination are embedded directly into participating nodes.

This dual design allows VIRGO to scale from experimental overlays to native, low-latency distributed infrastructures.

---

## Core Innovations

### 1. Virtual Hierarchical Tree
- Physical nodes can host **multiple virtual nodes** across layers.
- Enables massive logical scaling with minimal physical infrastructure.
- Decouples logical topology from physical deployment.

### 2. sVIRGO Non-Overlay Routing
- Hierarchical routing and failure recovery are **embedded natively in nodes**.
- Deterministic routing paths with predictable latency.
- Reduced overhead compared to traditional overlays.
- When IP  unavailable

### 3. Dynamic Role Assignment
- Nodes adapt roles across layers based on health, load, and topology.
- Supports highly dynamic environments (edge, mobile, intermittent links).
- Eliminates single points of failure through role redundancy.

### 4. Cross-Platform Rust Core
- Memory-safe, high-performance implementation.
- Reproducible builds across:
  - Linux
  - Windows
  - macOS
  - ARM / embedded platforms
- Designed for long-term maintainability and correctness.

--- 

## Societal & Industrial Impact

VIRGO and sVIRGO provide a **foundational framework for hierarchical distributed systems**, enabling scalable, multi-role coordination in a wide range of domains. Example applications include:

### 🏥 Healthcare Networks
- Cross-hospital collaboration and coordination.  
- **Privacy-preserving, scalable data aggregation**.  
- Supports regional and national medical alliances.

### 🤖 Federated & Distributed AI
- Secure, decentralized training without centralizing sensitive data.  
- Hierarchical aggregation ensures **scalability and robustness**.  
- Suitable for privacy-sensitive AI deployments in healthcare, finance, and smart cities.

### 🚚 Smart Logistics & Edge Systems
- Coordination of drones, IoT devices, and autonomous robots.  
- Low-latency hierarchical control for large-scale deployments.  
- Resilient operation under partial failures or network partitions.

> ⚡ Note: These examples illustrate potential applications. VIRGO and sVIRGO serve as a **core infrastructure for any distributed system that requires semantic-aware, multi-role, hierarchical coordination**.


## Research Vision

VIRGO is designed as:

- A **reference architecture** for hierarchical distributed systems.  
- A bridge between **theoretical distributed systems** and **real-world deployments**.  
- A **modular, reusable core** that can be integrated into diverse applications, supporting multi-role nodes, hierarchical routing, and fault tolerance.  
- A foundation for future research in areas such as:  
  - **Fault tolerance** and resilient distributed networks  
  - **Large-scale coordination** across millions of logical nodes  
  - **Edge computing** and IoT integration  
  - **Federated intelligence** and privacy-preserving distributed AI


## VIRGO as a Distributed OS

VIRGO and sVIRGO can be thought of as a **distributed operating system** for hierarchical networks. Just like a conventional OS abstracts hardware for applications, VIRGO abstracts **network hierarchy, multi-role node management, and fault-tolerant communication** for distributed applications.  

Key characteristics:

- **Modular Core:** Provides reusable services (routing, role assignment, failure recovery) as a standalone library, similar to how Linux exposes kernel modules.  
- **Multi-role Node Management:** Each physical node can host multiple logical nodes across tree layers, dynamically taking on roles from leaf to global root.  
- **Scalable Hierarchy:** Supports millions of logical nodes without requiring proportional physical resources.  
- **Cross-platform:** Rust-based core runs on Linux, Windows, macOS, and ARM architectures.  
- **Application Integration:** Applications can leverage the core for distributed computing, federated AI, IoT coordination, healthcare networks, and more, without implementing low-level network logic.  

> ⚡ Think of VIRGO as the **“kernel” for distributed hierarchical networks**, providing foundational services that can be reused across many domains and applications.

 

## Call to Collaborators

We welcome **researchers, engineers, and open-source contributors** to help shape VIRGO’s future.

You can contribute by:
- Developing the **Rust core**
- Building **simulators and testbeds**
- Creating **platform adapters** (IoT, edge, cloud)
- Exploring **formal models, proofs, and evaluations**

Together, we aim to establish VIRGO as a **foundational, citable, and long-lived architecture** in distributed systems research.

---

## License

**MIT License**  
Free and open for academic, industrial, and experimental use.

---

## Status

🚧 Actively evolving  
📦 Core architecture defined  
🧪 Prototyping and evaluation ongoing

---

## Contact & Community

- GitHub Issues & Discussions: *welcome*
- Research collaborations: *encouraged*

# VIRGO / sVIRGO Research and References

## 📚 References

### Core VIRGO / sVIRGO Publications

1. **Lican Huang**,  
   *VIRGO: Virtual Hierarchical Overlay Network for Scalable Grid Computing*,  
   in *Advances in Grid Computing (EGC 2005)*,  
   Lecture Notes in Computer Science, vol. 3470, Springer, 2005, pp. 911–921.  
   [DOI: 10.1007/11508380_93](https://doi.org/10.1007/11508380_93)

2. **Lican Huang**,  
   *Virtual Hierarchical Tree Grid Organizations (VIRGO)*,  
   in *Handbook of Research on P2P and Grid Systems for Service-Oriented Computing*,  
   IGI Global, 2010.  DOI: 10.4018/978-1-61520-686-5.ch030
   [IGI Global Link](https://www.igi-global.com/chapter/virtual-hierarchical-tree-grid-organizations/40824)

3. **Lican Huang**,  
   *Constructing Large Scale Cooperative Multi-Agent Systems from Semantic P2P Networks*,  
   Studies in Computational Intelligence, vol. 460, Springer, 2013, pp. 257–277.  
   [DOI: 10.1007/978-3-642-34952-2_11](https://doi.org/10.1007/978-3-642-34952-2_11)

4. **Lican Huang**,  
   *Private Virtual Tree Networks for Secure Multi-Tenant Environments Based on the VIRGO Overlay Network*,  
   arXiv preprint [arXiv:2512.15915](https://arxiv.org/abs/2512.15915), 2025.

5. **Lican Huang**,  
   *sVIRGO: A Scalable Virtual Tree Hierarchical Framework for Distributed Systems*,  
   arXiv preprint [arXiv:2602.02438](https://arxiv.org/abs/2602.02438), 2026.

---

### Related References

6. **Lican Huang**,  
   *Research on Scalable Architecture for e-Science Grid*,  
   PhD Thesis, School of Computer Science and Technology,  
   Zhejiang University, Nov. 2003. *(in Chinese)*

7. **Lican Huang**, Z. Wu, Y. Pan,  
   *Virtual and Dynamic Hierarchical Architecture for E-Science Grid*,  
   *International Journal of High Performance Computing Applications*,  
   vol. 17, no. 3, pp. 329–347, 2003.  
   [DOI: 10.1177/1094342003173007](https://doi.org/10.1177/1094342003173007)

---

### Supporting / Cooperative References

8. Yunliang Jiang, Yong Liu, Wenliang Huang, Lican Huang,  
   *Performance Analysis of a Mobile Agent Prototype System Based on VIRGO P2P Protocols*,  
   *Concurrency and Computation: Practice and Experience*, vol. 26, no. 2, pp. 447–467, 2013.  
   [DOI: 10.1002/cpe.3006](https://doi.org/10.1002/cpe.3006)

9. **Lican Huang**,  
   *High Performance Computation Based on Semantic P2P Network*,  
   in *2015 IEEE International Conference on Smart City/SocialCom/SustainCom/DataCom (SC2)*,  
   Chengdu, China, Dec. 19–21, 2015, pp. 1159–1162.  
   [DOI: 10.1109/SMARTCITY.2015.228](https://doi.org/10.1109/SMARTCITY.2015.228)

10. **Lican Huang**,  
    *A P2P Service Discovery Strategy Based on Content Catalogues*,  
    *Data Science Journal*, vol. 6, pp. 492–499, 2007.   DOI: 10.2481/dsj.6.S492
    [Link](https://datascience.codata.org/articles/dsj.6.S492)

 
