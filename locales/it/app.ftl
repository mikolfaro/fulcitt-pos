## App

app-title = POS Fulcitt
app-currency = { NUMBER($amount, style: "currency", currency: "eur", currencyDisplay: "symbol") }

app-message-generic-InvalidInput = Operazione non valida
app-message-generic-PrinterNotConfigured = Stampante non configurata

## POS

pos-cart-product-name = Prodotto
pos-cart-unit-price = Prezzo
pos-cart-quantity = Q.tà
pos-cart-title = Ordine attuale
pos-cart-toggle-refund-mode = Attiva storno
pos-cart-total-price = Totale

pos-reopen-cart-button = Riapri ordine
pos-clear-cart-button = Cancella ordine
pos-menu-title = POS

pos-messages-empty-cart = Nessun prodotto
pos-messages-loading-products = Caricamento prodotti...
pos-messages-cannot-process-transaction-with-no-items = Nessun prodotto nel carrello
pos-messages-invalid-price-for-product = Prezzo non valido {$price} per prodotto {$product}
pos-messages-invalid-quantity-for-product = Quantità non valida {$quantity} per prodotto {$product}
pos-messages-insufficient-amount = Importo pagato insufficiente

pos-products-title = Prodotti
pos-products-refunds-title = Rimborso cauzioni

pos-process-payment-button = Completa pagamento

pos-recap-amount-due = Dovuto
pos-recap-amount-payed = Pagato
pos-recap-change = Resto
pos-recap-subtotal = Subtotale
pos-recap-total = Totale
pos-recap-refund = Rimborso cauzione
pos-recap-cancellation = Storno

pos-reprint-tickets-button = Ristampa ultima vendita
pos-start-payment-button = Pagamento
pos-start-payment-card-button = Carta
pos-start-payment-cash-button = Contanti

## Reports

reports-clear-reports-button = Cancella report e dati sul venduto
reports-export-xlsx-button = Esporta report
reports-menu-title = Report
reports-title = Report di vendita

reports-export-product-name = Prodotto
reports-export-quantity = Q.tà
reports-export-sale-id = # Scontrino

reports-export-xlsx-currency-format = "#,##0.00 €"
reports-export-xlsx-date-time-format = "%Y-%M-%d %H:%m:%S"
reports-export-xlsx-unit-cost = Costo unitario
reports-export-xslx-invoices-tab-title = "Scontrini"
reports-export-xslx-invoices-details-tab-title = "Dettaglio"

reports-messages-loading-data = Caricamento dati...
reports-messages-no-data-available = Nessun dato sulle vendite disponibile
reports-messages-export-xlsx-completed = Report esportato

reports-payment-method-card = Carta
reports-payment-method-cash = Contanti

reports-sales-by-product-title = Prodotti venduti
reports-sales-by-product-product-name = Prodotto
reports-sales-by-product-quantity-sold = Qtà venduta
reports-sales-by-product-value-sold = Importo venduto

reports-today-sales-title = Vendite di oggi
reports-today-sales-invoice-number = Numero vendita
reports-today-sales-amount-sold = Importo venduto
reports-today-sales-payment-method = Metodo di pagamento
reports-today-sales-reprint-tickets-button = Ristampa vendita
reports-today-sales-sales-time = Orario vendita

reports-refunds-title = Resi cauzioni
reports-refunds-product-name = Prodotto
reports-refunds-quantity-refunded = Quantità
reports-refunds-value-refunded = Importo rimborsato

reports-date-filter-title = Filtra per data
reports-date-start-label = Data inizio
reports-date-end-label = Data fine
reports-date-today = Oggi
reports-date-last-7-days = Ultimi 7 giorni
reports-date-this-month = Questo mese
reports-date-all-time = Tutto il periodo

reports-stat-total-sales = Vendite totali nette
reports-stat-transactions-count = {$count ->
  [one] 1 transazione
  *[other] { $count } transazioni
}
reports-stat-total-refunds = Rimborsi totali
reports-stat-refunds-count = {$count ->
  [one] 1 articolo
  *[other] { $count } articoli
}
reports-stat-cash-payments = Pagamenti in contanti
reports-stat-cash-count = {$count ->
  [one] 1 transazione
  *[other] { $count } transazioni
}
reports-stat-card-payments = Pagamenti con carta
reports-stat-card-count = {$count ->
  [one] 1 transazione
  *[other] { $count } transazioni
}

reports-no-data = Nessun dato disponibile
reports-total = Totale
reports-messages-data-cleared = Dati cancellati con successo

## Settings

settings-menu-title = Impostazioni

settings-layout-font-size-label = Dimensione caratteri
settings-layout-font-size-large = Grande
settings-layout-font-size-normal = Normale
settings-layout-font-size-small = Piccolo
settings-layout-justify-center = Centrato
settings-layout-justify-label = Allineamento
settings-layout-justify-left = Sinistra
settings-layout-justify-right = Destra
settings-layout-menu-title = Layout
settings-layout-header-content-label = Contenuto
settings-layout-print-body-label = Stampa corpo
settings-layout-print-footer-label = Stampa piè di pagina
settings-layout-print-group-tickets-by-category = Raggruppa ticket per categoria
settings-layout-print-header-label = Stampa intestazione
settings-layout-save-button = Salva
settings-layout-title = Layout
Settings-layout-messages-layout-saved = Layout salvato

settings-printer-device-path-label = Stampante
settings-printer-menu-title = Stampante
settings-printer-title = Stampante
settings-printer-refresh-available-devices-button = Aggiorna elenco device
settings-printer-printer-selection-title = Selezione stampante
settings-printer-printer-selection-text = Assicurati che la stampante sia collegata e il percorso del device sia configurato correttamente.
settings-printer-save-button = Salva
settings-printer-test-print-button = Stampa di prova
settings-printer-text-to-be-printed-label = Testo da stampare
settings-printer-test-example = Ciao a tutti da Tauri! @ { DATETIME($date, dateStyle: "short", timeStyle: "short") }
settings-printer-type-selection-label = Tipo stampante
settings-printer-usb-device-selection-label = Device USB
settings-printer-virtual-printer-description = Una stampante virtuale non è una vera e propria "stampante". I ticket sono salvati nel file di testo riportato qui sotto.
settings-printer-virtual-printer-open-file-button = Apri file
settings-printer-virtual-printer-open-directory-button = Apri cartella
settings-printer-messages-device-cannot-be-empty = Il percorso del device non può essere vuoto.
settings-printer-messages-print-success = Stampa richiesta con successo
settings-printer-messages-printer-device-saved = Impostazioni salvate
settings-printer-messages-no-printer-selected = Nessuna stampante selezionata
settings-printer-messages-usb-device-not-selected = Device USB non selezionato
settings-printer-messages-no-text-to-print = Nessun testo da stampare
settings-printer-messages-opening-printer-file = Apertura file stampante...
settings-printer-messages-opening-app-folder = Apertura cartella applicazione...
settings-printer-no-printer-selected = Nessuna stampante selezionata
settings-printer-virtual-printer-output-title = Output stampante virtuale
settings-printer-network-printer-title = Stampante di rete
settings-printer-network-printer-address-label = Indirizzo IP
settings-printer-network-printer-port-label = Porta
settings-printer-test-printer-title = Test stampante
settings-printer-usb-device-selection = Selezione device USB

settings-products-actions = #
settings-products-add-product-button = Aggiungi
settings-products-cancel-edit-button = Annulla
settings-products-category = Categoria
settings-products-category-example = Griglia
settings-products-delete-product-button = Elimina
settings-products-edit-product-button = Modifica
settings-products-menu-title = Prodotti
settings-products-messages-incorrect-fields = Verifica i campi inseriti
settings-products-messages-loading-existing-products = Caricamento prodotti esistenti...
settings-products-messages-product-deleted = Prodotto eliminato
settings-products-messages-product-deleted = Prodotto aggiornato
settings-products-messages-price-must-be-positive = Il prezzo deve essere zero o più
settings-products-price = Prezzo
settings-products-price-example = 4,00 €
settings-products-product-name = Nome
settings-products-product-name-example = Panino con Salamella
settings-products-save-product-button = Salva
settings-products-title = Prodotti

settings-title = Impostazioni

settings-ui-menu-title = Interfaccia
settings-ui-title = Interfaccia utente

settings-ui-theme-label = Tema
settings-ui-theme-dark = Scuro
settings-ui-theme-light = Chiaro
settings-ui-theme-system = Sistema

settings-ui-zoom-label = Livello zoom
settings-ui-zoom-normal = Normale (100%)
settings-ui-zoom-large = Grande (150%)
settings-ui-zoom-xlarge = Molto grande (200%)

settings-ui-save-button = Salva
settings-ui-messages-settings-saved = Impostazioni interfaccia salvate
