-- Datos de venta sintéticos para validar el motor de puntos.
-- La copia de producción no incluye filas en pv.venta, así que insertamos
-- ventas controladas para un cliente real ya presente en adm.trc:
--
--   Cliente: MARIA GLADIYS IBAÑEZ PEREIRA
--   TRCID     = '000000000000001'
--   TRCNUMDOC = '40019552-3'   (venta.VENCLI referencia el documento)
--
-- Con la configuración por defecto (monto_por_punto = 1000):
--   Factura 1.500   -> FLOOR(1500/1000)  =  1 punto
--   Factura 25.000  -> FLOOR(25000/1000) = 25 puntos
--   Factura 99.000  -> ANULADA (VENEST='A'), NO otorga puntos
--   ------------------------------------------------------------
--   Saldo esperado  = 26 puntos  (valor 26 * 50 = 1.300 COP)

USE pv;

INSERT INTO venta (VENCIA, VENDOC, VENCLI, VENNOM, VENVAL, VENFEC, VENEST)
VALUES
  ('1', 900001, '40019552-3', 'MARIA GLADIYS IBAÑEZ',  1500, '2026-01-10', ''),
  ('1', 900002, '40019552-3', 'MARIA GLADIYS IBAÑEZ', 25000, '2026-02-15', ''),
  ('1', 900003, '40019552-3', 'MARIA GLADIYS IBAÑEZ', 99000, '2026-03-01', 'A');
