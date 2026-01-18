# NeXt Editor 🚀

**NeXt** è un compilatore di documenti scientifici che gira interamente nel browser.
Scritto in **Rust** e compilato in **WebAssembly**, offre un ambiente di scrittura sicuro, immediato e privo di latenza.

> **Zero Installazione. Zero Cloud. 100% Privacy.**

## ✨ Caratteristiche Principali

* **⚡ Core in Rust/WASM:** Prestazioni native direttamente nel browser.
* **🤖 AI-Ready:** Incolla direttamente risposte da ChatGPT, Gemini o Google Docs. Il motore riconosce automaticamente le formule matematiche (formato `$..$`) senza bisogno di riformattazione.
* **🧮 Rendering LaTeX:** Motore matematico KaTeX integrato per formule professionali.
* **📄 Esportazione PDF:** Stampa documenti puliti con un click.
* **🔒 Privacy-First:** I dati non lasciano mai il tuo dispositivo.

---

## 📖 Guida Rapida

### 1. Metadati
Inizia il documento definendo il titolo:
```text
@title{Titolo del Documento}

### 2. Testo e Stile
Scrivi normalmente per i paragrafi.
Usa gli asterischi per il **grassetto**.
Usa i trattini per le liste:
- Primo punto
- Secondo punto

### 3. Matematica (LaTeX)
NeXt supporta formule complesse tra i dollari (puoi incollare da ChatGPT!):
$ E = mc^2 $

Integrali e frazioni:
$ \int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi} $

### 4. Tabelle
Crea tabelle usando le barre verticali:
| Comando | Descrizione |
| :--- | :--- |
| ** | Grassetto |
| $ | Matematica |
| @title | Titolo Doc |

